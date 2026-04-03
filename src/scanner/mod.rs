pub mod external;
pub mod known_path;
pub mod project_scan;
pub mod size;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::config::exclude::is_excluded;
use crate::rules::{Rule, RuleKind, ScanEvent, ScanStats};

/// Starts scanning in a background thread, sending events through the returned receiver.
pub fn start_scan(
    rules: Vec<Rule>,
    scan_roots: Vec<std::path::PathBuf>,
    exclude_patterns: Vec<String>,
) -> mpsc::Receiver<ScanEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();
        let mut total_items: usize = 0;
        let mut total_size: u64 = 0;

        // Phase 1: Scan known paths
        let known_path_rules: Vec<&Rule> = rules
            .iter()
            .filter(|r| matches!(r.kind, RuleKind::KnownPath { .. }))
            .collect();

        for item in known_path::scan(&known_path_rules) {
            if is_excluded(&item.path, &exclude_patterns) {
                continue;
            }
            total_size += item.size;
            total_items += 1;
            let _ = tx.send(ScanEvent::ItemFound(item));
        }

        // Phase 2: Scan project artifacts
        let project_rules: Vec<&Rule> = rules
            .iter()
            .filter(|r| matches!(r.kind, RuleKind::ProjectScan { .. }))
            .collect();

        if !project_rules.is_empty() {
            for root in &scan_roots {
                let _ = tx.send(ScanEvent::Progress(format!(
                    "Scanning {}...",
                    crate::util::path::shorten_path(root)
                )));
                for item in project_scan::scan(&project_rules, root, &tx) {
                    if is_excluded(&item.path, &exclude_patterns) {
                        continue;
                    }
                    total_size += item.size;
                    total_items += 1;
                    let _ = tx.send(ScanEvent::ItemFound(item));
                }
            }
        }

        // Phase 3: External command rules (Docker, simulators, etc.)
        let external_rules: Vec<&Rule> = rules
            .iter()
            .filter(|r| matches!(r.kind, RuleKind::ExternalCommand { .. }))
            .collect();

        if !external_rules.is_empty() {
            let _ = tx.send(ScanEvent::Progress("Checking external tools...".into()));
            for item in external::scan(&external_rules, &tx) {
                total_size += item.size;
                total_items += 1;
                let _ = tx.send(ScanEvent::ItemFound(item));
            }
        }

        let _ = tx.send(ScanEvent::Complete(ScanStats {
            total_items,
            total_size,
            duration: start.elapsed(),
        }));
    });

    rx
}

use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;

use jwalk::WalkDirGeneric;

use crate::rules::{JunkItem, Rule, RuleKind, ScanEvent};
use crate::util::path::shorten_path;

use super::size::dir_size;

/// Target definition extracted from ProjectScan rules for efficient matching.
struct ScanTarget {
    rule_id: String,
    rule_name: String,
    category: crate::rules::Category,
    risk: crate::rules::Risk,
    confirm_siblings: Option<Vec<String>>,
    suffix_match: bool,
    clean_command: Option<Vec<String>>,
}

/// Scans the filesystem under `root` using all ProjectScan rules simultaneously.
pub fn scan(rules: &[&Rule], root: &Path, tx: &mpsc::Sender<ScanEvent>) -> Vec<JunkItem> {
    // Build lookup structures for efficient matching during walk.
    let mut exact_targets: std::collections::HashMap<String, Vec<ScanTarget>> =
        std::collections::HashMap::new();
    let mut suffix_targets: Vec<(String, ScanTarget)> = Vec::new();

    // Collect all target directory names so we can prevent descent into them.
    let mut target_dir_names: HashSet<String> = HashSet::new();

    for rule in rules {
        let (target_names, confirm_sibling) = match &rule.kind {
            RuleKind::ProjectScan {
                target_names,
                confirm_sibling,
            } => (target_names, confirm_sibling),
            _ => continue,
        };

        for name in target_names {
            target_dir_names.insert(name.clone());

            let target = ScanTarget {
                rule_id: rule.id.clone(),
                rule_name: rule.name.clone(),
                category: rule.category,
                risk: rule.risk,
                confirm_siblings: confirm_sibling.clone(),
                suffix_match: name.starts_with('.')
                    && name != ".venv"
                    && name != ".tox"
                    && name != ".bundle",
                clean_command: rule.clean_command.clone(),
            };

            if target.suffix_match {
                suffix_targets.push((name.clone(), target));
            } else {
                exact_targets.entry(name.clone()).or_default().push(target);
            }
        }
    }

    // Directories to never descend into (not interesting for project artifacts).
    let skip_dirs: HashSet<&str> = HashSet::from([
        ".git",
        ".hg",
        ".svn",
        "Library",
        "Applications",
        ".Trash",
        ".cache",
        "Pictures",
        "Music",
        "Movies",
        "Photos Library.photoslibrary",
        "site-packages",
        "dist-packages",
        "__pypackages__",
    ]);

    let mut items = Vec::new();
    let mut progress_counter = 0u32;

    let walk = WalkDirGeneric::<((), bool)>::new(root)
        .skip_hidden(false)
        .process_read_dir(move |_depth, _path, _state, children| {
            for child in children.iter_mut().flatten() {
                if child.file_type().is_dir() {
                    let name = child.file_name.to_string_lossy();

                    // Skip directories we're not interested in scanning inside.
                    if skip_dirs.contains(name.as_ref()) {
                        child.read_children_path = None;
                        child.client_state = true;
                    }

                    // Don't descend into target directories (node_modules, target, etc.)
                    // — we want to REPORT them but not walk inside them.
                    if target_dir_names.contains(name.as_ref()) {
                        child.read_children_path = None;
                    }

                    // Don't descend into .app bundles.
                    if name.ends_with(".app") {
                        child.read_children_path = None;
                        child.client_state = true;
                    }
                }
            }
        });

    for entry in walk {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Skip entries marked for skipping.
        if entry.client_state {
            continue;
        }

        if !entry.file_type().is_dir() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy();
        let entry_path = entry.path();

        // Send progress updates periodically.
        progress_counter += 1;
        if progress_counter.is_multiple_of(500) {
            let _ = tx.send(ScanEvent::Progress(format!(
                "Scanning {}...",
                shorten_path(&entry_path)
            )));
        }

        // Check exact name matches.
        if let Some(targets) = exact_targets.get(file_name.as_ref()) {
            for target in targets {
                if check_sibling(&entry_path, &target.confirm_siblings) {
                    let size = dir_size(&entry_path);
                    if size > 0 {
                        items.push(JunkItem {
                            rule_id: target.rule_id.clone(),
                            rule_name: target.rule_name.clone(),
                            category: target.category,
                            risk: target.risk,
                            path: entry_path.clone(),
                            size,
                            last_modified: entry_path
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok()),
                            clean_command: target.clean_command.clone(),
                        });
                        break;
                    }
                }
            }
        }

        // Check suffix matches (e.g., ".egg-info").
        for (suffix, target) in &suffix_targets {
            if file_name.ends_with(suffix.as_str())
                && check_sibling(&entry_path, &target.confirm_siblings)
            {
                let size = dir_size(&entry_path);
                if size > 0 {
                    items.push(JunkItem {
                        rule_id: target.rule_id.clone(),
                        rule_name: target.rule_name.clone(),
                        category: target.category,
                        risk: target.risk,
                        path: entry_path.clone(),
                        size,
                        last_modified: entry_path.metadata().ok().and_then(|m| m.modified().ok()),
                        clean_command: target.clean_command.clone(),
                    });
                    break;
                }
            }
        }
    }

    items
}

/// Checks if any of the required sibling files exist in the parent directory.
fn check_sibling(path: &Path, siblings: &Option<Vec<String>>) -> bool {
    let Some(siblings) = siblings else {
        return true;
    };
    let Some(parent) = path.parent() else {
        return false;
    };
    siblings.iter().any(|s| parent.join(s).exists())
}

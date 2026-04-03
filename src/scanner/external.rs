use std::process::Command;
use std::sync::mpsc;

use crate::rules::{JunkItem, Rule, RuleKind, ScanEvent};

/// Try to scan external command rules. Checks if the service is available
/// before attempting detection. Sends warnings for unavailable services.
pub fn scan(rules: &[&Rule], tx: &mpsc::Sender<ScanEvent>) -> Vec<JunkItem> {
    let mut items = Vec::new();

    for rule in rules {
        let (detect_cmd, clean_cmd) = match &rule.kind {
            RuleKind::ExternalCommand {
                detect_cmd,
                clean_cmd,
            } => (detect_cmd, clean_cmd),
            _ => continue,
        };

        if detect_cmd.is_empty() {
            continue;
        }

        let result = Command::new(&detect_cmd[0])
            .args(&detect_cmd[1..])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output();

        match result {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let size = parse_size_from_output(&stdout, &rule.id);

                if size > 0 {
                    items.push(JunkItem {
                        rule_id: rule.id.clone(),
                        rule_name: rule.name.clone(),
                        category: rule.category,
                        risk: rule.risk,
                        path: std::path::PathBuf::from(format!("[{}]", detect_cmd.join(" "))),
                        size,
                        last_modified: None,
                        clean_command: Some(clean_cmd.clone()),
                    });
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let reason = stderr.lines().next().unwrap_or("unknown error").to_string();

                // Distinguish "not running" from other failures.
                let msg = if reason.contains("daemon")
                    || reason.contains("connect")
                    || reason.contains("sock")
                {
                    format!("{}: service not running — start it to scan", rule.name)
                } else {
                    format!("{}: {}", rule.name, reason)
                };
                let _ = tx.send(ScanEvent::Error(msg));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Binary not installed — completely silent, this is expected.
            }
            Err(e) => {
                let _ = tx.send(ScanEvent::Error(format!("{}: {e}", rule.name)));
            }
        }
    }

    items
}

/// Parse size from `docker system df` or similar output.
fn parse_size_from_output(stdout: &str, rule_id: &str) -> u64 {
    match rule_id {
        "docker-system" => parse_docker_df(stdout),
        _ => {
            // For unknown formats, report nominal 1 byte to indicate "something found".
            if !stdout.trim().is_empty() { 1 } else { 0 }
        }
    }
}

/// Parse `docker system df` output to get total reclaimable size.
//
// Example output:
//   TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
//   Images          5         1         2.3GB     1.8GB (78%)
//   Containers      3         1         62.5MB    62.5MB (100%)
//   Local Volumes   2         1         300MB     150MB (50%)
//   Build Cache     10        0         500MB     500MB
fn parse_docker_df(stdout: &str) -> u64 {
    let mut total: u64 = 0;
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            for part in parts.iter().rev() {
                if let Some(bytes) = parse_docker_size(part) {
                    total += bytes;
                    break;
                }
            }
        }
    }
    total
}

/// Parse Docker size strings like "2.3GB", "500MB", "1.5kB".
fn parse_docker_size(s: &str) -> Option<u64> {
    if s.starts_with('(') {
        return None;
    }

    let s = s.trim();
    if s == "0B" || s == "0" {
        return Some(0);
    }

    let (num_str, multiplier) = if let Some(n) = s.strip_suffix("TB") {
        (n, 1_000_000_000_000u64)
    } else if let Some(n) = s.strip_suffix("GB") {
        (n, 1_000_000_000)
    } else if let Some(n) = s.strip_suffix("MB") {
        (n, 1_000_000)
    } else if let Some(n) = s.strip_suffix("kB") {
        (n, 1_000)
    } else if let Some(n) = s.strip_suffix('B') {
        (n, 1)
    } else {
        return None;
    };

    num_str
        .parse::<f64>()
        .ok()
        .map(|n| (n * multiplier as f64) as u64)
}

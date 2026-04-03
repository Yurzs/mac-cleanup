use std::fs;
use std::process::Command;
use std::sync::mpsc;
use std::time::Instant;

use crate::rules::{CleanEvent, CleanStats, JunkItem};

/// Execute deletion of selected items, sending progress events.
pub fn execute(items: Vec<JunkItem>, tx: mpsc::Sender<CleanEvent>) {
    let start = Instant::now();
    let mut deleted_count = 0usize;
    let mut deleted_size = 0u64;
    let mut failed_count = 0usize;

    for item in &items {
        let _ = tx.send(CleanEvent::Deleting(item.path.clone()));

        let result = if let Some(cmd) = &item.clean_command {
            run_clean_command(cmd).or_else(|_| fs_delete(&item.path))
        } else {
            fs_delete(&item.path)
        };

        match result {
            Ok(()) => {
                crate::cleaner::safety::log_deletion(&item.path, item.size, true);
                deleted_count += 1;
                deleted_size += item.size;
                let _ = tx.send(CleanEvent::Deleted {
                    path: item.path.clone(),
                    size: item.size,
                });
            }
            Err(e) => {
                crate::cleaner::safety::log_deletion(&item.path, item.size, false);
                failed_count += 1;
                let _ = tx.send(CleanEvent::Failed {
                    path: item.path.clone(),
                    error: e,
                });
            }
        }
    }

    let _ = tx.send(CleanEvent::Complete(CleanStats {
        deleted_count,
        deleted_size,
        failed_count,
        duration: start.elapsed(),
    }));
}

/// Try running a native cleanup command.
fn run_clean_command(cmd: &[String]) -> Result<(), String> {
    if cmd.is_empty() {
        return Err("empty command".into());
    }
    let status = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("{}: {e}", cmd[0]))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{} exited with {status}", cmd[0]))
    }
}

/// Filesystem-based deletion fallback.
fn fs_delete(path: &std::path::Path) -> Result<(), String> {
    let result = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    result.map_err(|e| e.to_string())
}

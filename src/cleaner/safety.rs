use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Write a deletion event to the audit log.
pub fn log_deletion(path: &Path, size: u64, success: bool) {
    let log_dir = dirs::home_dir()
        .map(|h| h.join(".local/state/mac-cleanup"))
        .unwrap_or_else(|| "/tmp/mac-cleanup".into());

    let _ = fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("cleanup.log");

    let status = if success { "OK" } else { "FAIL" };
    let timestamp = chrono_lite_timestamp();

    let entry = format!(
        "{timestamp}  {status}  {size:>12}  {}\n",
        path.display()
    );

    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = f.write_all(entry.as_bytes());
    }
}

fn chrono_lite_timestamp() -> String {
    use std::time::SystemTime;
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = d.as_secs();
    // Simple ISO-ish timestamp without pulling in chrono.
    // Format: seconds since epoch (good enough for a log).
    format!("{secs}")
}

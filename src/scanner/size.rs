use std::os::unix::fs::MetadataExt;
use std::path::Path;
use walkdir::WalkDir;

/// Computes the actual disk usage of a path recursively.
/// Uses st_blocks * 512 to match `du` output (real disk usage,
/// accounting for APFS compression, clones, and sparse files).
pub fn dir_size(path: &Path) -> u64 {
    if path.is_file() {
        return path.metadata().map(|m| m.blocks() * 512).unwrap_or(0);
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.blocks() * 512).unwrap_or(0))
        .sum()
}

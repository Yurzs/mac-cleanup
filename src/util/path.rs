use std::path::{Path, PathBuf};

/// Expands `~` at the start of a path to the user's home directory.
pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    } else if path == "~"
        && let Some(home) = dirs::home_dir() {
            return home;
        }
    PathBuf::from(path)
}

/// Shortens a path by replacing the home directory prefix with `~`.
pub fn shorten_path(path: &Path) -> String {
    if let Some(home) = dirs::home_dir()
        && let Ok(suffix) = path.strip_prefix(&home) {
            return format!("~/{}", suffix.display());
        }
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/Library/Caches");
        assert!(expanded.is_absolute());
        assert!(expanded.ends_with("Library/Caches"));
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let path = "/usr/local/bin";
        assert_eq!(expand_tilde(path), PathBuf::from(path));
    }

    #[test]
    fn test_shorten_path() {
        if let Some(home) = dirs::home_dir() {
            let full = home.join("Library/Caches");
            assert_eq!(shorten_path(&full), "~/Library/Caches");
        }
    }
}

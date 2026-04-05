use std::path::Path;

/// Check if a path matches any of the exclude patterns.
pub fn is_excluded(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    patterns.iter().any(|pattern| {
        if pattern.contains('*') {
            glob_match(pattern, &path_str)
        } else {
            path_str.contains(pattern.as_str())
        }
    })
}

/// Glob matching supporting `*` (any within segment) and `**` (any path).
/// Handles multiple wildcards via recursive matching.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let mut px = 0; // pattern index
    let mut tx = 0; // text index
    let mut star_px = usize::MAX; // last * position in pattern
    let mut star_tx = 0; // text position when last * was hit

    let pat = pattern.as_bytes();
    let txt = text.as_bytes();

    while tx < txt.len() {
        if px < pat.len() && pat[px] == b'*' {
            // Consume consecutive *'s (including **).
            while px < pat.len() && pat[px] == b'*' {
                px += 1;
            }
            star_px = px;
            star_tx = tx;
        } else if px < pat.len() && (pat[px] == txt[tx] || pat[px] == b'?') {
            px += 1;
            tx += 1;
        } else if star_px != usize::MAX {
            // Backtrack: advance the text position after the last *.
            star_tx += 1;
            tx = star_tx;
            px = star_px;
        } else {
            return false;
        }
    }

    // Consume trailing *'s in pattern.
    while px < pat.len() && pat[px] == b'*' {
        px += 1;
    }

    px == pat.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_plain_substring() {
        let path = PathBuf::from("/Users/me/.cargo/registry/src");
        assert!(is_excluded(&path, &["cargo".into()]));
        assert!(!is_excluded(&path, &["gradle".into()]));
    }

    #[test]
    fn test_star_glob() {
        assert!(glob_match("*.log", "app.log"));
        assert!(glob_match("*.log", "error.log"));
        assert!(!glob_match("*.log", "app.txt"));
    }

    #[test]
    fn test_double_star() {
        assert!(glob_match(
            "**/node_modules",
            "/Users/me/project/node_modules"
        ));
        assert!(glob_match("**/__pycache__", "/a/b/c/__pycache__"));
    }

    #[test]
    fn test_star_in_middle() {
        assert!(glob_match(
            "~/Projects/*/node_modules",
            "~/Projects/foo/node_modules"
        ));
        // In exclude patterns, * matches across path separators (same as **).
        assert!(glob_match(
            "~/Projects/*/node_modules",
            "~/Projects/foo/bar/node_modules"
        ));
    }

    #[test]
    fn test_double_star_middle() {
        assert!(glob_match("~/work/**/target", "~/work/a/b/c/target"));
        assert!(glob_match("~/*/foo/**/bar", "~/x/foo/a/b/bar"));
    }

    #[test]
    fn test_multiple_stars() {
        assert!(glob_match("*.log*", "app.log.1"));
        assert!(glob_match("*.log*", "app.log"));
    }
}

use std::path::Path;

use crate::rules::{Rule, RuleKind};
use crate::util::path::expand_tilde;

/// Errors found during profile validation.
#[derive(Debug)]
pub struct ValidationError {
    pub rule_id: String,
    pub reason: String,
}

/// Validate all rules in an external profile. Returns errors for each invalid rule.
pub fn validate_rules(rules: &[Rule]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in rules {
        errors.extend(validate_rule(rule));
    }
    errors
}

fn validate_rule(rule: &Rule) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Validate paths in KnownPath rules.
    if let RuleKind::KnownPath { paths } = &rule.kind {
        for path_str in paths {
            if let Some(err) = validate_path(path_str) {
                errors.push(ValidationError {
                    rule_id: rule.id.clone(),
                    reason: format!("path {path_str}: {err}"),
                });
            }
        }
    }

    // Validate the parent dir in GlobKeepLatest rules.
    if let RuleKind::GlobKeepLatest { parent } = &rule.kind
        && let Some(err) = validate_path(parent)
    {
        errors.push(ValidationError {
            rule_id: rule.id.clone(),
            reason: format!("parent {parent}: {err}"),
        });
    }

    // Validate the parent dir + name pattern in KnownPathGlob rules.
    if let RuleKind::KnownPathGlob {
        parent,
        name_pattern,
    } = &rule.kind
    {
        if let Some(err) = validate_glob_parent(parent) {
            errors.push(ValidationError {
                rule_id: rule.id.clone(),
                reason: format!("parent {parent}: {err}"),
            });
        }
        if name_pattern.is_empty() || name_pattern.contains('/') {
            errors.push(ValidationError {
                rule_id: rule.id.clone(),
                reason: format!(
                    "name_pattern {name_pattern}: must be a non-empty single file name pattern"
                ),
            });
        }
    }

    // Validate clean_command if present.
    if let Some(cmd) = &rule.clean_command
        && let Some(err) = validate_command(cmd)
    {
        errors.push(ValidationError {
            rule_id: rule.id.clone(),
            reason: format!("clean_command: {err}"),
        });
    }

    // Validate ExternalCommand clean_cmd.
    if let RuleKind::ExternalCommand { clean_cmd, .. } = &rule.kind
        && let Some(err) = validate_command(clean_cmd)
    {
        errors.push(ValidationError {
            rule_id: rule.id.clone(),
            reason: format!("clean_cmd: {err}"),
        });
    }

    errors
}

/// Validate a KnownPathGlob parent directory. Accepts either home-relative
/// paths (same rules as [`validate_path`]) or a small allowlist of absolute
/// system directories whose children are safe to glob-match against.
fn validate_glob_parent(path_str: &str) -> Option<String> {
    // Allowlist of absolute parents where glob matching is explicitly permitted.
    const ALLOWED_ABSOLUTE_PARENTS: &[&str] = &["/Applications"];
    if ALLOWED_ABSOLUTE_PARENTS.contains(&path_str) {
        return None;
    }
    validate_path(path_str)
}

/// Validate a target path is safe.
fn validate_path(path_str: &str) -> Option<String> {
    // Must start with ~/ (home-relative).
    if !path_str.starts_with("~/") && path_str != "~" {
        return Some("must be under home directory (~/)".into());
    }

    // Block path traversal.
    if path_str.contains("..") {
        return Some("path traversal (..) not allowed".into());
    }

    // Resolve and check against blocked paths.
    let resolved = expand_tilde(path_str);

    // Block overly broad paths (home dir itself, or critical personal dirs).
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return Some("cannot determine home directory".into()),
    };

    if resolved == home {
        return Some("cannot target home directory itself".into());
    }

    let blocked_subdirs = [
        "Documents",
        "Desktop",
        "Downloads",
        "Pictures",
        "Music",
        "Movies",
        ".ssh",
        ".gnupg",
        ".aws",
        ".kube",
    ];

    for dir in &blocked_subdirs {
        if resolved == home.join(dir) || resolved.starts_with(home.join(dir).join("")) {
            return Some(format!("~/{dir} is a protected directory"));
        }
    }

    // Block system paths (in case ~ expansion somehow resolves to system).
    let blocked_prefixes: &[&str] = &[
        "/System",
        "/usr",
        "/bin",
        "/sbin",
        "/var",
        "/etc",
        "/tmp",
        "/private",
        "/Applications",
        "/Library",
        "/opt",
    ];

    let resolved_str = resolved.to_string_lossy();
    for prefix in blocked_prefixes {
        if resolved_str.starts_with(prefix) {
            // Allow ~/Library/* but not /Library/*
            if *prefix == "/Library"
                && resolved_str.starts_with(&format!("{}/Library", home.display()))
            {
                continue;
            }
            return Some(format!("system path {prefix} is blocked"));
        }
    }

    None
}

/// Validate a cleanup command is safe.
fn validate_command(cmd: &[String]) -> Option<String> {
    if cmd.is_empty() {
        return Some("empty command".into());
    }

    let binary = &cmd[0];

    // Extract just the binary name (strip path prefix if any).
    let binary_name = Path::new(binary)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| binary.clone());

    // Allowlist of known-safe binaries that cleanup profiles might use.
    let allowed_binaries = [
        "go",
        "cargo",
        "npm",
        "yarn",
        "pnpm",
        "pip",
        "pip3",
        "uv",
        "conda",
        "brew",
        "gem",
        "bundle",
        "bundler",
        "pod",
        "gradle",
        "mvn",
        "docker",
        "podman",
        "finch",
        "container",
        "tmutil",
        "xcrun",
        "xcodebuild",
        "flutter",
        "dart",
        "composer",
        "jupyter",
        "poetry",
        "pdm",
        "rye",
    ];

    if !allowed_binaries.contains(&binary_name.as_str()) {
        return Some(format!(
            "'{binary_name}' is not in the allowed command list. \
             Allowed: {}",
            allowed_binaries.join(", ")
        ));
    }

    // Block shell metacharacters in all arguments.
    let dangerous_chars = [';', '|', '&', '$', '`', '(', ')', '{', '}', '<', '>'];
    for arg in cmd {
        if arg.chars().any(|c| dangerous_chars.contains(&c)) {
            return Some(format!("argument '{arg}' contains shell metacharacters"));
        }
    }

    // Block arguments that look like they're trying to delete everything.
    let dangerous_args = ["/", "/*", "~", "~/*", "-rf /", "--all /"];
    for arg in &cmd[1..] {
        let lower = arg.to_lowercase();
        if dangerous_args.contains(&lower.as_str()) {
            return Some(format!("argument '{arg}' targets dangerous path"));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path() {
        assert!(validate_path("~/Library/Caches/pip").is_none());
        assert!(validate_path("~/.cargo/registry/cache").is_none());
        assert!(validate_path("~/go/pkg/mod/cache").is_none());
    }

    #[test]
    fn test_blocked_paths() {
        assert!(validate_path("/").is_some());
        assert!(validate_path("/usr/local").is_some());
        assert!(validate_path("~").is_some());
        assert!(validate_path("~/Documents").is_some());
        assert!(validate_path("~/Documents/projects").is_some());
        assert!(validate_path("~/.ssh").is_some());
        assert!(validate_path("~/../etc/passwd").is_some());
    }

    #[test]
    fn test_valid_commands() {
        assert!(validate_command(&["go".into(), "clean".into(), "-cache".into()]).is_none());
        assert!(validate_command(&["brew".into(), "cleanup".into()]).is_none());
        assert!(
            validate_command(&[
                "npm".into(),
                "cache".into(),
                "clean".into(),
                "--force".into()
            ])
            .is_none()
        );
    }

    #[test]
    fn test_blocked_commands() {
        assert!(validate_command(&["rm".into(), "-rf".into(), "/".into()]).is_some());
        assert!(validate_command(&["sudo".into(), "rm".into()]).is_some());
        assert!(validate_command(&["sh".into(), "-c".into(), "bad".into()]).is_some());
        assert!(validate_command(&["curl".into(), "http://evil.com".into()]).is_some());
        assert!(validate_command(&[]).is_some());
    }

    #[test]
    fn test_shell_metacharacters_blocked() {
        assert!(validate_command(&["go".into(), "clean; rm -rf /".into()]).is_some());
        assert!(validate_command(&["npm".into(), "$(evil)".into()]).is_some());
        assert!(validate_command(&["brew".into(), "cleanup | cat".into()]).is_some());
    }
}

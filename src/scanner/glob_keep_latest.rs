use std::collections::HashMap;
use std::path::PathBuf;

use crate::rules::{JunkItem, Rule, RuleKind};
use crate::util::path::expand_tilde;

use super::size::dir_size;

/// Scans all GlobKeepLatest rules and returns found items.
///
/// For each rule, reads the parent directory, matches child entries against
/// the `<alphabetic family><numeric version>` pattern, groups by family, and
/// emits all non-latest versions in each family as stale JunkItems. Families
/// with only one version present are left alone.
pub fn scan(rules: &[&Rule]) -> Vec<JunkItem> {
    let mut items = Vec::new();

    for rule in rules {
        let parent_str = match &rule.kind {
            RuleKind::GlobKeepLatest { parent } => parent,
            _ => continue,
        };

        let parent = expand_tilde(parent_str);
        let Ok(entries) = std::fs::read_dir(&parent) else {
            continue;
        };

        // Group matching child directories by family name.
        let mut by_family: HashMap<String, Vec<(Vec<u32>, PathBuf)>> = HashMap::new();
        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if !file_type.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some((family, version)) = parse_family_version(&name) else {
                continue;
            };
            by_family
                .entry(family)
                .or_default()
                .push((version, entry.path()));
        }

        // In each family, sort by version and flag all but the latest.
        for (_family, mut versions) in by_family {
            if versions.len() < 2 {
                continue;
            }
            versions.sort_by(|a, b| a.0.cmp(&b.0));
            versions.pop(); // Drop the latest — keep it.

            for (_v, path) in versions {
                let size = dir_size(&path);
                if size == 0 {
                    continue;
                }
                let last_modified = path.metadata().ok().and_then(|m| m.modified().ok());
                items.push(JunkItem {
                    rule_id: rule.id.clone(),
                    rule_name: rule.name.clone(),
                    category: rule.category,
                    risk: rule.risk,
                    path,
                    size,
                    last_modified,
                    clean_command: rule.clean_command.clone(),
                });
            }
        }
    }

    items
}

/// Parse a directory name of the form `<alphabetic family><numeric version>`.
///
/// The family is the longest alphabetic (ASCII letters only) prefix, and the
/// version is the remainder parsed as dot-separated u32 components. Returns
/// `None` if the name doesn't fit the pattern.
///
/// Examples:
///   `"IntelliJIdea2025.3"`  → Some(("IntelliJIdea", [2025, 3]))
///   `"PyCharm2026.1.2"`     → Some(("PyCharm", [2026, 1, 2]))
///   `"Toolbox"`             → None (no version)
///   `"2025.3"`              → None (no family)
fn parse_family_version(name: &str) -> Option<(String, Vec<u32>)> {
    let split_at = name.chars().position(|c| c.is_ascii_digit())?;
    if split_at == 0 {
        return None;
    }
    let (family, version_str) = name.split_at(split_at);
    if !family.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let version: Vec<u32> = version_str
        .split('.')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    if version.is_empty() {
        return None;
    }
    Some((family.to_string(), version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_jetbrains_style_names() {
        assert_eq!(
            parse_family_version("IntelliJIdea2025.3"),
            Some(("IntelliJIdea".into(), vec![2025, 3]))
        );
        assert_eq!(
            parse_family_version("PyCharm2026.1"),
            Some(("PyCharm".into(), vec![2026, 1]))
        );
        assert_eq!(
            parse_family_version("GoLand2025.3.2"),
            Some(("GoLand".into(), vec![2025, 3, 2]))
        );
        assert_eq!(
            parse_family_version("RustRover2026.1"),
            Some(("RustRover".into(), vec![2026, 1]))
        );
    }

    #[test]
    fn rejects_non_matching_names() {
        assert_eq!(parse_family_version("Toolbox"), None);
        assert_eq!(parse_family_version("Daemon"), None);
        assert_eq!(parse_family_version("2025.3"), None);
        assert_eq!(parse_family_version("IntelliJ_2025.3"), None);
        assert_eq!(parse_family_version("PyCharm2025.3alpha"), None);
    }

    #[test]
    fn version_tuples_sort_numerically() {
        let mut versions = vec![
            (vec![2025, 9], "a"),
            (vec![2025, 10], "b"),
            (vec![2026, 1], "c"),
            (vec![2025, 3], "d"),
        ];
        versions.sort_by(|x, y| x.0.cmp(&y.0));
        let sorted: Vec<&str> = versions.iter().map(|(_, s)| *s).collect();
        assert_eq!(sorted, vec!["d", "a", "b", "c"]);
    }
}

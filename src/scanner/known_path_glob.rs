use crate::config::exclude::glob_match;
use crate::rules::{JunkItem, Rule, RuleKind};
use crate::util::path::expand_tilde;

use super::size::dir_size;

/// Scans all KnownPathGlob rules and returns found items.
///
/// For each rule, reads the parent directory and matches every child entry's
/// name against `name_pattern`. Matching entries (files or directories) are
/// sized and emitted individually.
pub fn scan(rules: &[&Rule]) -> Vec<JunkItem> {
    let mut items = Vec::new();

    for rule in rules {
        let (parent_str, pattern) = match &rule.kind {
            RuleKind::KnownPathGlob {
                parent,
                name_pattern,
            } => (parent, name_pattern),
            _ => continue,
        };

        let parent = expand_tilde(parent_str);
        let Ok(entries) = std::fs::read_dir(&parent) else {
            continue;
        };

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !glob_match(pattern, &name) {
                continue;
            }

            let path = entry.path();
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

    items
}

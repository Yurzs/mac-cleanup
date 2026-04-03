use crate::rules::{JunkItem, Rule, RuleKind};
use crate::util::path::expand_tilde;

use super::size::dir_size;

/// Scans all KnownPath rules and returns found items.
pub fn scan(rules: &[&Rule]) -> Vec<JunkItem> {
    let mut items = Vec::new();

    for rule in rules {
        let paths = match &rule.kind {
            RuleKind::KnownPath { paths } => paths,
            _ => continue,
        };

        for path_str in paths {
            let path = expand_tilde(path_str);
            if !path.exists() {
                continue;
            }

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

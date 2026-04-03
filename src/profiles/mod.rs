pub mod builtin;
pub mod toml_format;
pub mod validate;

use std::collections::HashSet;

use crate::rules::Rule;
use crate::util::path::expand_tilde;

/// A condition checked to determine if a profile is relevant to this system.
#[derive(Debug, Clone)]
pub enum DetectCondition {
    /// A command must exist on PATH.
    CommandOnPath(String),
    /// A path must exist on disk.
    PathExists(String),
}

/// A cleanup profile grouping related rules.
#[derive(Debug, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub detect: Vec<DetectCondition>,
    pub builtin: bool,
    /// For built-in profiles: IDs of rules from all_rules() to include.
    pub rule_ids: Vec<String>,
    /// For external profiles: new rules defined in the profile.
    pub extra_rules: Vec<Rule>,
}

/// Result of auto-detection for a single profile.
#[derive(Debug, Clone)]
pub struct ProfileStatus {
    pub id: String,
    pub name: String,
    pub description: String,
    pub builtin: bool,
    pub detected: bool,
    pub matched_reasons: Vec<String>,
}

/// Load all profiles (built-in + external TOML).
pub fn load_all_profiles() -> Vec<Profile> {
    let mut profiles = builtin::all();
    profiles.extend(toml_format::load_external_profiles());
    profiles
}

/// Detect which profiles are relevant to this system.
pub fn detect_profiles(profiles: &[Profile]) -> Vec<ProfileStatus> {
    profiles
        .iter()
        .map(|p| {
            let mut matched = Vec::new();
            for cond in &p.detect {
                match cond {
                    DetectCondition::CommandOnPath(cmd) => {
                        if which_exists(cmd) {
                            matched.push(format!("{cmd} on PATH"));
                        }
                    }
                    DetectCondition::PathExists(path_str) => {
                        let path = expand_tilde(path_str);
                        if path.exists() {
                            matched.push(format!("{path_str} exists"));
                        }
                    }
                }
            }
            // Empty detect list = always active (e.g., "general" profile).
            let detected = p.detect.is_empty() || !matched.is_empty();
            ProfileStatus {
                id: p.id.clone(),
                name: p.name.clone(),
                description: p.description.clone(),
                builtin: p.builtin,
                detected,
                matched_reasons: matched,
            }
        })
        .collect()
}

/// Resolve which rules to use given selected profile IDs.
/// Collects rule_ids from matching profiles, filters all_rules(), and merges extra_rules.
pub fn resolve_rules(profiles: &[Profile], selected_ids: &[String]) -> Vec<Rule> {
    let selected_set: HashSet<&str> = selected_ids.iter().map(|s| s.as_str()).collect();

    let mut wanted_rule_ids: HashSet<String> = HashSet::new();
    let mut extra_rules: Vec<Rule> = Vec::new();

    for profile in profiles {
        if !selected_set.contains(profile.id.as_str()) {
            continue;
        }
        for id in &profile.rule_ids {
            wanted_rule_ids.insert(id.clone());
        }
        extra_rules.extend(profile.extra_rules.clone());
    }

    // Filter built-in rules by the collected IDs.
    let mut rules: Vec<Rule> = crate::rules::all_rules()
        .into_iter()
        .filter(|r| wanted_rule_ids.contains(&r.id))
        .collect();

    // Deduplicate extra rules against already-included IDs.
    let existing_ids: HashSet<String> = rules.iter().map(|r| r.id.clone()).collect();
    for rule in extra_rules {
        if !existing_ids.contains(&rule.id) {
            rules.push(rule);
        }
    }

    rules
}

/// Check if a command exists on PATH.
fn which_exists(cmd: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| {
                let full = dir.join(cmd);
                full.is_file()
            })
        })
        .unwrap_or(false)
}

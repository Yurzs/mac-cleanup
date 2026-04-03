use serde::Deserialize;

use crate::rules::{Category, Risk, Rule, RuleKind};

use super::{DetectCondition, Profile};

/// Top-level TOML structure for an external profile.
#[derive(Debug, Deserialize)]
struct TomlProfile {
    profile: TomlProfileMeta,
    #[serde(default)]
    rules: Vec<TomlRule>,
}

#[derive(Debug, Deserialize)]
struct TomlProfileMeta {
    name: String,
    description: String,
    #[serde(default)]
    detect: Vec<TomlDetectCondition>,
}

#[derive(Debug, Deserialize)]
struct TomlDetectCondition {
    command_on_path: Option<String>,
    path_exists: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TomlRule {
    id: String,
    name: String,
    category: Category,
    risk: Risk,
    description: String,
    kind: RuleKind,
    #[serde(default)]
    clean_command: Option<Vec<String>>,
}

/// Load external profiles from ~/.config/mac-cleanup/profiles/*.toml.
pub fn load_external_profiles() -> Vec<Profile> {
    let profile_dir = match dirs::config_dir() {
        Some(dir) => dir.join("mac-cleanup/profiles"),
        None => return vec![],
    };

    if !profile_dir.is_dir() {
        return vec![];
    }

    let mut profiles = Vec::new();

    let entries = match std::fs::read_dir(&profile_dir) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            match load_profile_file(&path) {
                Ok(profile) => profiles.push(profile),
                Err(e) => {
                    log::warn!("Failed to load profile {}: {e}", path.display());
                }
            }
        }
    }

    profiles
}

fn load_profile_file(path: &std::path::Path) -> anyhow::Result<Profile> {
    let content = std::fs::read_to_string(path)?;
    let toml_profile: TomlProfile = toml::from_str(&content)?;

    // Derive profile ID from filename.
    let id = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    // Convert detect conditions.
    let detect: Vec<DetectCondition> = toml_profile
        .profile
        .detect
        .into_iter()
        .filter_map(|c| {
            if let Some(cmd) = c.command_on_path {
                Some(DetectCondition::CommandOnPath(cmd))
            } else {
                c.path_exists.map(DetectCondition::PathExists)
            }
        })
        .collect();

    // Convert rules.
    let extra_rules: Vec<Rule> = toml_profile
        .rules
        .into_iter()
        .map(|r| Rule {
            id: r.id,
            name: r.name,
            category: r.category,
            kind: r.kind,
            risk: r.risk,
            description: r.description,
            clean_command: r.clean_command,
            profile_id: Some(id.clone()),
        })
        .collect();

    // Validate all rules in the external profile.
    let validation_errors = super::validate::validate_rules(&extra_rules);
    if !validation_errors.is_empty() {
        let details: Vec<String> = validation_errors
            .iter()
            .map(|e| format!("  rule '{}': {}", e.rule_id, e.reason))
            .collect();
        anyhow::bail!(
            "profile '{}' failed validation:\n{}",
            id,
            details.join("\n")
        );
    }

    Ok(Profile {
        id,
        name: toml_profile.profile.name,
        description: toml_profile.profile.description,
        detect,
        builtin: false,
        rule_ids: vec![],
        extra_rules,
    })
}

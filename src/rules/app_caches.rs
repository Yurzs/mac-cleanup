use super::{Category, Risk, Rule, RuleKind};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "chrome-cache".into(),
            name: "Chrome cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/Google/Chrome".into()],
            },
            risk: Risk::Safe,
            description: "Google Chrome browser cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "firefox-cache".into(),
            name: "Firefox cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/Firefox".into()],
            },
            risk: Risk::Safe,
            description: "Mozilla Firefox browser cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "safari-cache".into(),
            name: "Safari cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/com.apple.Safari".into()],
            },
            risk: Risk::Safe,
            description: "Apple Safari browser cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "spotify-cache".into(),
            name: "Spotify cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/com.spotify.client".into()],
            },
            risk: Risk::Safe,
            description: "Spotify streaming cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "slack-cache".into(),
            name: "Slack cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/com.tinyspeck.slackmacgap".into()],
            },
            risk: Risk::Safe,
            description: "Slack application cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "discord-cache".into(),
            name: "Discord cache".into(),
            category: Category::AppCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/com.hnc.Discord".into()],
            },
            risk: Risk::Safe,
            description: "Discord application cache.".into(),
            clean_command: None,
            profile_id: None,
        },
    ]
}

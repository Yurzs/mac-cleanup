use super::{Category, Risk, Rule, RuleKind};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "xcode-derived".into(),
            name: "Xcode DerivedData".into(),
            category: Category::SystemJunk,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Developer/Xcode/DerivedData".into()],
            },
            risk: Risk::Safe,
            description: "Xcode build intermediates. Rebuilt automatically on next build.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "xcode-archives".into(),
            name: "Xcode Archives".into(),
            category: Category::SystemJunk,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Developer/Xcode/Archives".into()],
            },
            risk: Risk::Caution,
            description: "Xcode build archives. May contain signed release builds you need.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "system-logs".into(),
            name: "User logs".into(),
            category: Category::SystemJunk,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Logs".into()],
            },
            risk: Risk::Safe,
            description: "Application and system log files.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "trash".into(),
            name: "Trash".into(),
            category: Category::SystemJunk,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.Trash".into()],
            },
            risk: Risk::Safe,
            description: "Items already in the Trash. Equivalent to 'Empty Trash'.".into(),
            clean_command: None,
            profile_id: None,
        },
    ]
}

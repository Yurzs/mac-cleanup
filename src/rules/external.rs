use super::{Category, Risk, Rule, RuleKind};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "docker-system".into(),
            name: "Docker system".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                detect_cmd: vec!["docker".into(), "system".into(), "df".into()],
                clean_cmd: vec![
                    "docker".into(),
                    "system".into(),
                    "prune".into(),
                    "-af".into(),
                ],
            },
            risk: Risk::Caution,
            description: "Docker images, containers, and build cache. Requires Docker to be running.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "ios-simulators".into(),
            name: "iOS Simulators (unavailable)".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                detect_cmd: vec![
                    "xcrun".into(),
                    "simctl".into(),
                    "list".into(),
                    "devices".into(),
                    "unavailable".into(),
                ],
                clean_cmd: vec![
                    "xcrun".into(),
                    "simctl".into(),
                    "delete".into(),
                    "unavailable".into(),
                ],
            },
            risk: Risk::Safe,
            description: "iOS Simulator runtimes for old/unavailable OS versions.".into(),
            clean_command: None,
            profile_id: None,
        },
    ]
}

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
            description:
                "Docker images, containers, and build cache. Requires Docker to be running.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "podman-system".into(),
            name: "Podman system".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                detect_cmd: vec!["podman".into(), "system".into(), "df".into()],
                clean_cmd: vec![
                    "podman".into(),
                    "system".into(),
                    "prune".into(),
                    "-af".into(),
                ],
            },
            risk: Risk::Caution,
            description:
                "Podman images, containers, and build cache. Requires 'podman machine' to be running."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "finch-system".into(),
            name: "Finch system".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                detect_cmd: vec!["finch".into(), "system".into(), "df".into()],
                clean_cmd: vec![
                    "finch".into(),
                    "system".into(),
                    "prune".into(),
                    "-af".into(),
                ],
            },
            risk: Risk::Caution,
            description:
                "Finch (AWS) images, containers, and build cache. Requires the Finch VM to be running."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "apple-container-system".into(),
            name: "Apple container system".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                // Apple's `container` CLI (released WWDC 2025). Using `list --all`
                // as detection because it's the most broadly available subcommand;
                // the prune subcommand may vary by version — adjust if needed.
                detect_cmd: vec!["container".into(), "list".into(), "--all".into()],
                clean_cmd: vec![
                    "container".into(),
                    "system".into(),
                    "prune".into(),
                    "--force".into(),
                ],
            },
            risk: Risk::Caution,
            description:
                "Apple container runtime (WWDC 2025) images and containers. \
                 Requires the container service to be running."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "tm-local-snapshots".into(),
            name: "Time Machine local snapshots".into(),
            category: Category::External,
            kind: RuleKind::ExternalCommand {
                detect_cmd: vec!["tmutil".into(), "listlocalsnapshots".into(), "/".into()],
                // Request 1 TB of free space with urgency 4 → thins all thinnable snapshots.
                clean_cmd: vec![
                    "tmutil".into(),
                    "thinlocalsnapshots".into(),
                    "/".into(),
                    "1000000000000".into(),
                    "4".into(),
                ],
            },
            risk: Risk::Safe,
            description:
                "APFS local Time Machine snapshots. Often hold GBs of 'System Data' hostage even \
                 with no external TM disk connected. macOS auto-thins them under disk pressure, \
                 but this forces immediate reclaim."
                    .into(),
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

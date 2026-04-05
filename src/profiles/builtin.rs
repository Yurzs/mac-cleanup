use super::{DetectCondition, Profile};

/// Returns all built-in profiles.
pub fn all() -> Vec<Profile> {
    vec![general(), developer(), ios(), android(), devops()]
}

fn general() -> Profile {
    Profile {
        id: "general".into(),
        name: "General".into(),
        description: "System junk, browser caches, app caches (always active)".into(),
        detect: vec![], // Empty = always active.
        builtin: true,
        rule_ids: vec![
            "system-logs".into(),
            "trash".into(),
            "quicklook-thumbnails".into(),
            "ios-device-backups".into(),
            "ios-ipsw-updates".into(),
            "macos-installer-apps".into(),
            "tm-local-snapshots".into(),
            "chrome-cache".into(),
            "firefox-cache".into(),
            "safari-cache".into(),
            "spotify-cache".into(),
            "slack-cache".into(),
            "discord-cache".into(),
        ],
        extra_rules: vec![],
    }
}

fn developer() -> Profile {
    Profile {
        id: "developer".into(),
        name: "Developer".into(),
        description: "Build caches and project artifacts for software development".into(),
        detect: vec![
            DetectCondition::CommandOnPath("node".into()),
            DetectCondition::CommandOnPath("cargo".into()),
            DetectCondition::CommandOnPath("go".into()),
            DetectCondition::CommandOnPath("python3".into()),
            DetectCondition::CommandOnPath("ruby".into()),
        ],
        builtin: true,
        rule_ids: vec![
            "pip-cache".into(),
            "uv-cache".into(),
            "poetry-cache".into(),
            "conda-pkgs".into(),
            "hatch-cache".into(),
            "pipenv-cache".into(),
            "go-build-cache".into(),
            "go-mod-cache".into(),
            "cargo-registry-cache".into(),
            "cargo-registry-src".into(),
            "npm-cache".into(),
            "yarn-cache".into(),
            "pnpm-store".into(),
            "homebrew-cache".into(),
            "node-modules".into(),
            "rust-target".into(),
            "python-venv".into(),
            "python-pycache".into(),
            "python-tox".into(),
            "python-eggs".into(),
            "ruby-bundle".into(),
            "cpp-build".into(),
            "jetbrains-cache".into(),
            "jetbrains-stale-versions".into(),
            "playwright-browsers".into(),
            "copilot-cache".into(),
            "docker-install-cache".into(),
        ],
        extra_rules: vec![],
    }
}

fn ios() -> Profile {
    Profile {
        id: "ios".into(),
        name: "iOS/macOS".into(),
        description: "Xcode, CocoaPods, and iOS Simulator caches".into(),
        detect: vec![
            DetectCondition::CommandOnPath("xcrun".into()),
            DetectCondition::PathExists("~/Library/Developer/Xcode".into()),
        ],
        builtin: true,
        rule_ids: vec![
            "cocoapods-cache".into(),
            "swiftpm-cache".into(),
            "carthage-cache".into(),
            "mint-cache".into(),
            "xcode-derived".into(),
            "xcode-archives".into(),
            "xcode-app-cache".into(),
            "xcode-simulator-caches".into(),
            "xcode-device-support".into(),
            "xcode-device-logs".into(),
            "ios-simulators".into(),
        ],
        extra_rules: vec![],
    }
}

fn android() -> Profile {
    Profile {
        id: "android".into(),
        name: "Android".into(),
        description: "Gradle, Maven, and Android SDK caches".into(),
        detect: vec![
            DetectCondition::PathExists("~/.gradle".into()),
            DetectCondition::CommandOnPath("gradle".into()),
        ],
        builtin: true,
        rule_ids: vec![
            "gradle-cache".into(),
            "maven-cache".into(),
            "android-sdk-system-images".into(),
            "java-gradle-build".into(),
            "java-maven-target".into(),
        ],
        extra_rules: vec![],
    }
}

fn devops() -> Profile {
    Profile {
        id: "devops".into(),
        name: "DevOps".into(),
        description: "Docker, container, and orchestration caches".into(),
        detect: vec![
            DetectCondition::CommandOnPath("docker".into()),
            DetectCondition::CommandOnPath("podman".into()),
            DetectCondition::CommandOnPath("finch".into()),
            DetectCondition::CommandOnPath("container".into()),
        ],
        builtin: true,
        rule_ids: vec![
            "docker-system".into(),
            "podman-system".into(),
            "finch-system".into(),
            "apple-container-system".into(),
            "lens-cache".into(),
        ],
        extra_rules: vec![],
    }
}

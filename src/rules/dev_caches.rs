use super::{Category, Risk, Rule, RuleKind};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "pip-cache".into(),
            name: "pip cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/pip".into()],
            },
            risk: Risk::Safe,
            description: "Python pip download cache. Will be re-downloaded as needed.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "uv-cache".into(),
            name: "uv cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/uv".into()],
            },
            risk: Risk::Safe,
            description: "uv package manager cache. Will be re-downloaded as needed.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "go-build-cache".into(),
            name: "Go build cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/go-build".into()],
            },
            risk: Risk::Safe,
            description: "Go compilation cache. Equivalent to 'go clean -cache'.".into(),
            clean_command: Some(vec!["go".into(), "clean".into(), "-cache".into()]),
            profile_id: None,
        },
        Rule {
            id: "go-mod-cache".into(),
            name: "Go module cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/go/pkg/mod/cache".into()],
            },
            risk: Risk::Safe,
            description: "Go module download cache. Equivalent to 'go clean -modcache'.".into(),
            clean_command: Some(vec!["go".into(), "clean".into(), "-modcache".into()]),
            profile_id: None,
        },
        Rule {
            id: "cargo-registry-cache".into(),
            name: "Cargo registry cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.cargo/registry/cache".into()],
            },
            risk: Risk::Safe,
            description: "Cargo crate download cache. Will be re-downloaded on next build.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "cargo-registry-src".into(),
            name: "Cargo registry src".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.cargo/registry/src".into()],
            },
            risk: Risk::Safe,
            description: "Extracted Cargo crate sources. Will be re-extracted from cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "npm-cache".into(),
            name: "npm cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.npm/_cacache".into()],
            },
            risk: Risk::Safe,
            description: "npm package download cache. Equivalent to 'npm cache clean --force'."
                .into(),
            clean_command: Some(vec![
                "npm".into(),
                "cache".into(),
                "clean".into(),
                "--force".into(),
            ]),
            profile_id: None,
        },
        Rule {
            id: "yarn-cache".into(),
            name: "Yarn cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/Yarn".into()],
            },
            risk: Risk::Safe,
            description: "Yarn package manager cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "pnpm-store".into(),
            name: "pnpm store".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/pnpm/store".into()],
            },
            risk: Risk::Caution,
            description:
                "pnpm content-addressable store. Uses hardlinks — other projects may reference it."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "cocoapods-cache".into(),
            name: "CocoaPods cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/CocoaPods".into()],
            },
            risk: Risk::Safe,
            description: "CocoaPods pod download cache.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "gradle-cache".into(),
            name: "Gradle caches".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.gradle/caches".into()],
            },
            risk: Risk::Safe,
            description: "Gradle dependency and build caches. Will be re-downloaded on next build."
                .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "maven-cache".into(),
            name: "Maven repository".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.m2/repository".into()],
            },
            risk: Risk::Safe,
            description: "Maven local repository cache. Will be re-downloaded on next build."
                .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "homebrew-cache".into(),
            name: "Homebrew cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/Homebrew".into()],
            },
            risk: Risk::Safe,
            description:
                "Homebrew downloaded bottles and source archives. Equivalent to 'brew cleanup'."
                    .into(),
            clean_command: Some(vec!["brew".into(), "cleanup".into(), "--prune=all".into()]),
            profile_id: None,
        },
    ]
}

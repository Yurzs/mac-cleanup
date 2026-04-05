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
            description: "Python pip download cache. Equivalent to 'pip cache purge'.".into(),
            clean_command: Some(vec!["pip".into(), "cache".into(), "purge".into()]),
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
            description: "uv package manager cache. Equivalent to 'uv cache clean'.".into(),
            clean_command: Some(vec!["uv".into(), "cache".into(), "clean".into()]),
            profile_id: None,
        },
        Rule {
            id: "poetry-cache".into(),
            name: "Poetry cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/pypoetry".into()],
            },
            risk: Risk::Safe,
            description: "Poetry package manager cache. Will be re-downloaded as needed.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "conda-pkgs".into(),
            name: "conda package cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec![
                    "~/.conda/pkgs".into(),
                    "~/miniconda3/pkgs".into(),
                    "~/anaconda3/pkgs".into(),
                    "~/miniforge3/pkgs".into(),
                    "~/mambaforge/pkgs".into(),
                ],
            },
            risk: Risk::Safe,
            description:
                "conda downloaded package tarballs and unused packages. Equivalent to 'conda clean --all --yes'."
                    .into(),
            clean_command: Some(vec![
                "conda".into(),
                "clean".into(),
                "--all".into(),
                "--yes".into(),
            ]),
            profile_id: None,
        },
        Rule {
            id: "hatch-cache".into(),
            name: "Hatch cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/hatch".into()],
            },
            risk: Risk::Safe,
            description: "Hatch build tool cache. Will be re-downloaded as needed.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "pipenv-cache".into(),
            name: "Pipenv cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/pipenv".into()],
            },
            risk: Risk::Safe,
            description: "Pipenv package resolver and download cache. Equivalent to 'pipenv --clear'."
                .into(),
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
            id: "swiftpm-cache".into(),
            name: "SwiftPM cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/org.swift.swiftpm".into()],
            },
            risk: Risk::Safe,
            description:
                "Swift Package Manager shared cache (downloaded packages and repos). \
                 Re-downloaded on next 'swift build' or Xcode package resolve."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "carthage-cache".into(),
            name: "Carthage cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/org.carthage.CarthageKit".into()],
            },
            risk: Risk::Safe,
            description:
                "Carthage downloaded frameworks and dependencies. Re-downloaded on 'carthage update'."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "mint-cache".into(),
            name: "Mint cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/.mint/build".into(), "~/.mint/packages".into()],
            },
            risk: Risk::Safe,
            description:
                "Mint (Swift CLI tool manager) built binaries and source packages. \
                 Rebuilt on next 'mint run' or 'mint install'."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "jetbrains-cache".into(),
            name: "JetBrains IDE caches".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/JetBrains".into()],
            },
            risk: Risk::Safe,
            description:
                "JetBrains IDE caches and indexes (IntelliJ, PyCharm, GoLand, WebStorm, etc.). \
                 IDEs will re-index on next open."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "jetbrains-stale-versions".into(),
            name: "JetBrains stale IDE versions".into(),
            category: Category::DevCache,
            kind: RuleKind::GlobKeepLatest {
                parent: "~/Library/Application Support/JetBrains".into(),
            },
            risk: Risk::Caution,
            description:
                "Per-version JetBrains IDE config + plugin directories for previous releases \
                 (e.g. IntelliJIdea2025.3 when 2026.1 exists). The newest release in each family \
                 is always kept. Only remove if you've committed to the newer version — \
                 rolling back the IDE would lose these settings and plugins."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "playwright-browsers".into(),
            name: "Playwright browsers".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/ms-playwright".into()],
            },
            risk: Risk::Safe,
            description:
                "Playwright downloaded browser binaries (Chromium, Firefox, WebKit). \
                 Restored with 'npx playwright install'."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "copilot-cache".into(),
            name: "GitHub Copilot cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/copilot".into()],
            },
            risk: Risk::Safe,
            description: "GitHub Copilot completion cache. Regenerated as needed.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "docker-install-cache".into(),
            name: "Docker installer cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Application Support/com.docker.install".into()],
            },
            risk: Risk::Safe,
            description:
                "Docker Desktop installer staging directory. Safe to remove; Docker will re-stage on next update."
                    .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "lens-cache".into(),
            name: "Lens updater cache".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Caches/lens-desktop-updater".into()],
            },
            risk: Risk::Safe,
            description:
                "Lens (Kubernetes IDE) auto-updater cache. Re-downloaded on next update check."
                    .into(),
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
            id: "android-sdk-system-images".into(),
            name: "Android SDK system images".into(),
            category: Category::DevCache,
            kind: RuleKind::KnownPath {
                paths: vec!["~/Library/Android/sdk/system-images".into()],
            },
            risk: Risk::Caution,
            description:
                "Android emulator system disk images. Often multi-GB per API level. \
                 Re-downloadable via Android Studio's AVD Manager — but deleting invalidates \
                 existing AVD snapshots that depend on them."
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

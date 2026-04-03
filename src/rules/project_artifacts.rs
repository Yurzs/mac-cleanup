use super::{Category, Risk, Rule, RuleKind};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "node-modules".into(),
            name: "node_modules".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec!["node_modules".into()],
                confirm_sibling: Some(vec!["package.json".into()]),
            },
            risk: Risk::Safe,
            description: "Node.js dependencies. Restored with 'npm install' or 'yarn install'."
                .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "rust-target".into(),
            name: "Rust target".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec!["target".into()],
                confirm_sibling: Some(vec!["Cargo.toml".into()]),
            },
            risk: Risk::Safe,
            description: "Rust build artifacts. Restored with 'cargo build'.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "python-venv".into(),
            name: "Python venv".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec![".venv".into(), "venv".into()],
                confirm_sibling: Some(vec![
                    "pyproject.toml".into(),
                    "setup.py".into(),
                    "setup.cfg".into(),
                    "requirements.txt".into(),
                ]),
            },
            risk: Risk::Safe,
            description: "Python virtual environment. Recreate with 'python -m venv' or 'uv venv'."
                .into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "python-pycache".into(),
            name: "__pycache__".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec!["__pycache__".into()],
                confirm_sibling: None,
            },
            risk: Risk::Safe,
            description: "Python bytecode cache. Automatically regenerated.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "python-tox".into(),
            name: ".tox".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec![".tox".into()],
                confirm_sibling: Some(vec!["tox.ini".into(), "setup.cfg".into()]),
            },
            risk: Risk::Safe,
            description: "tox test environments. Recreated with 'tox'.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "python-eggs".into(),
            name: "*.egg-info".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec![".egg-info".into()],
                confirm_sibling: Some(vec!["setup.py".into(), "pyproject.toml".into()]),
            },
            risk: Risk::Safe,
            description: "Python egg metadata. Regenerated on install.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "java-gradle-build".into(),
            name: "Gradle build".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec!["build".into()],
                confirm_sibling: Some(vec!["build.gradle".into(), "build.gradle.kts".into()]),
            },
            risk: Risk::Safe,
            description: "Gradle build output. Restored with 'gradle build'.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "java-maven-target".into(),
            name: "Maven target".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec!["target".into()],
                confirm_sibling: Some(vec!["pom.xml".into()]),
            },
            risk: Risk::Safe,
            description: "Maven build output. Restored with 'mvn package'.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "ruby-bundle".into(),
            name: "Bundler vendor".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec![".bundle".into()],
                confirm_sibling: Some(vec!["Gemfile".into()]),
            },
            risk: Risk::Safe,
            description: "Ruby Bundler cache. Restored with 'bundle install'.".into(),
            clean_command: None,
            profile_id: None,
        },
        Rule {
            id: "cpp-build".into(),
            name: "CMake build".into(),
            category: Category::ProjectArtifact,
            kind: RuleKind::ProjectScan {
                target_names: vec![
                    "build".into(),
                    "cmake-build-debug".into(),
                    "cmake-build-release".into(),
                ],
                confirm_sibling: Some(vec!["CMakeLists.txt".into()]),
            },
            risk: Risk::Safe,
            description: "C/C++ CMake build directory. Restored with 'cmake --build'.".into(),
            clean_command: None,
            profile_id: None,
        },
    ]
}

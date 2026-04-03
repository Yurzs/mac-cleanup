pub mod app_caches;
pub mod dev_caches;
pub mod external;
pub mod project_artifacts;
pub mod system_junk;

use std::fmt;
use std::path::PathBuf;
use std::time::SystemTime;

/// Category of junk item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
pub enum Category {
    DevCache,
    ProjectArtifact,
    SystemJunk,
    AppCache,
    External,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DevCache => write!(f, "Developer Caches"),
            Self::ProjectArtifact => write!(f, "Project Artifacts"),
            Self::SystemJunk => write!(f, "System Junk"),
            Self::AppCache => write!(f, "App Caches"),
            Self::External => write!(f, "External"),
        }
    }
}

/// Risk level for deleting an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Risk {
    /// Safe to delete — will be re-downloaded/rebuilt as needed.
    Safe,
    /// Use caution — may contain data worth keeping.
    Caution,
    /// Dangerous — may cause data loss.
    Dangerous,
}

impl fmt::Display for Risk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safe => write!(f, "Safe"),
            Self::Caution => write!(f, "Caution"),
            Self::Dangerous => write!(f, "Dangerous"),
        }
    }
}

/// How a rule discovers junk.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleKind {
    /// Fixed known path(s) — no scanning needed, just stat.
    KnownPath {
        paths: Vec<String>,
    },
    /// Requires walking a filesystem tree to discover.
    ProjectScan {
        /// Directory names that identify the junk (e.g., "node_modules", "target").
        target_names: Vec<String>,
        /// Optional sibling file(s) that must exist in the parent to confirm
        /// this is a real project artifact (e.g., "package.json", "Cargo.toml").
        /// If multiple are provided, ANY match confirms.
        confirm_sibling: Option<Vec<String>>,
    },
    /// Requires invoking an external command.
    ExternalCommand {
        detect_cmd: Vec<String>,
        clean_cmd: Vec<String>,
    },
}

/// A rule that defines a type of junk to detect.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub kind: RuleKind,
    pub risk: Risk,
    pub description: String,
    /// Optional native cleanup command (e.g., ["go", "clean", "-cache"]).
    /// If set, the cleaner will try this before falling back to filesystem deletion.
    pub clean_command: Option<Vec<String>>,
    /// Which profile contributed this rule (None for default/all).
    pub profile_id: Option<String>,
}

/// A discovered junk item.
#[derive(Debug, Clone)]
pub struct JunkItem {
    pub rule_id: String,
    pub rule_name: String,
    pub category: Category,
    pub risk: Risk,
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: Option<SystemTime>,
    /// Native cleanup command carried from the Rule.
    pub clean_command: Option<Vec<String>>,
}

/// Events sent from the scanner to the TUI.
#[derive(Debug)]
pub enum ScanEvent {
    /// A new junk item was discovered.
    ItemFound(JunkItem),
    /// Progress update — currently scanning this path.
    Progress(String),
    /// Scanning completed.
    Complete(ScanStats),
    /// An error occurred during scanning.
    Error(String),
}

/// Statistics about a completed scan.
#[derive(Debug, Clone)]
pub struct ScanStats {
    pub total_items: usize,
    pub total_size: u64,
    pub duration: std::time::Duration,
}

/// Events sent from the cleaner to the TUI.
#[derive(Debug)]
pub enum CleanEvent {
    /// Started deleting an item.
    Deleting(PathBuf),
    /// Successfully deleted an item.
    Deleted { path: PathBuf, size: u64 },
    /// Failed to delete an item.
    Failed { path: PathBuf, error: String },
    /// All cleaning completed.
    Complete(CleanStats),
}

/// Statistics about a completed cleanup.
#[derive(Debug, Clone)]
pub struct CleanStats {
    pub deleted_count: usize,
    pub deleted_size: u64,
    pub failed_count: usize,
    pub duration: std::time::Duration,
}

/// Returns all built-in rules.
pub fn all_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    rules.extend(dev_caches::rules());
    rules.extend(project_artifacts::rules());
    rules.extend(system_junk::rules());
    rules.extend(app_caches::rules());
    rules.extend(external::rules());
    rules
}

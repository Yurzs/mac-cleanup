use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mac-cleanup", version, about = "macOS disk cleanup tool for developers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Disable interactive TUI — print a table instead.
    #[arg(long)]
    pub no_tui: bool,

    /// Output results as JSON.
    #[arg(long)]
    pub json: bool,

    /// Actually delete selected items (required for --no-tui mode cleanup).
    #[arg(short = 'x', long)]
    pub execute: bool,

    /// Skip confirmation prompt (use with --execute).
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Filter by category (dev-cache, project-artifact, system-junk, app-cache, external).
    #[arg(long, value_delimiter = ',')]
    pub category: Option<Vec<String>>,

    /// Activate specific profiles (comma-separated: developer, ios, android, devops, general).
    #[arg(long, value_delimiter = ',')]
    pub profile: Option<Vec<String>>,

    /// Auto-detect relevant profiles based on installed tools.
    #[arg(long)]
    pub auto_detect: bool,

    /// Directories to scan for project artifacts [default: ~].
    #[arg(long, value_delimiter = ',')]
    pub scan_roots: Option<Vec<PathBuf>>,

    /// Maximum depth for project artifact scanning.
    #[arg(long, default_value = "10")]
    pub max_depth: usize,

    /// Exclude paths matching these glob patterns.
    #[arg(long, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage cleanup profiles.
    Profiles {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileAction {
    /// List available profiles and their auto-detect status.
    List,
}

impl Cli {
    pub fn scan_roots(&self) -> Vec<PathBuf> {
        if let Some(roots) = &self.scan_roots {
            roots
                .iter()
                .map(|r| crate::util::path::expand_tilde(&r.to_string_lossy()))
                .collect()
        } else {
            dirs::home_dir().into_iter().collect()
        }
    }

    pub fn category_filter(&self) -> Option<Vec<crate::rules::Category>> {
        use crate::rules::Category;
        self.category.as_ref().map(|cats| {
            cats.iter()
                .filter_map(|c| match c.as_str() {
                    "dev-cache" | "dev" => Some(Category::DevCache),
                    "project-artifact" | "project" => Some(Category::ProjectArtifact),
                    "system-junk" | "system" => Some(Category::SystemJunk),
                    "app-cache" | "app" => Some(Category::AppCache),
                    "external" => Some(Category::External),
                    other => {
                        eprintln!(
                            "Warning: unknown category '{other}'. \
                             Valid: dev-cache, project-artifact, system-junk, app-cache, external"
                        );
                        None
                    }
                })
                .collect()
        })
    }
}

pub mod exclude;

use std::path::PathBuf;

use serde::Deserialize;

/// Optional config loaded from ~/.config/mac-cleanup/config.toml.
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub scan_roots: Vec<PathBuf>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Config {
    /// Attempt to load config from the standard location.
    /// Returns default config if file doesn't exist.
    pub fn load() -> Self {
        let path = dirs::config_dir()
            .map(|c| c.join("mac-cleanup/config.toml"))
            .unwrap_or_default();

        if !path.exists() {
            return Self::default();
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: failed to read {}: {e}", path.display());
                return Self::default();
            }
        };

        match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: failed to parse {}: {e}", path.display());
                Self::default()
            }
        }
    }
}

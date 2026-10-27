use crate::types::{Complexity, DevelopmentType, JargonLevel};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub dev_type: DevelopmentType,
    pub jargon_level: JargonLevel,
    pub complexity: Complexity,
    pub alerts_enabled: bool,
    pub project_name: String,
    pub minimal_output: bool,
    pub team_activity: bool,
    pub framework: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            dev_type: DevelopmentType::Backend,
            jargon_level: JargonLevel::Medium,
            complexity: Complexity::Medium,
            alerts_enabled: false,
            project_name: "distributed-cluster".to_string(),
            minimal_output: false,
            team_activity: false,
            framework: String::new(),
        }
    }
}

impl SessionConfig {
    pub fn config_file_path() -> Option<PathBuf> {
        let base_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config"))
            })?;
        Some(base_dir.join("rust-stakeholder").join("config.toml"))
    }

    pub fn load_saved() -> Option<Self> {
        let path = Self::config_file_path()?;
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn save(&self) -> Result<PathBuf, String> {
        let path = Self::config_file_path()
            .ok_or_else(|| "Could not determine config directory".to_string())?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config dir: {}", e))?;
        }

        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, toml_str)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(path)
    }
}

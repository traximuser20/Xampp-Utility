use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub xampp_path: PathBuf,
    pub backup_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let xampp_path = PathBuf::from("C:\\xampp");
        let backup_path = if PathBuf::from("D:\\").exists() {
            PathBuf::from("D:\\xampp_backups")
        } else {
            PathBuf::from("C:\\xampp_backups")
        };

        Self {
            xampp_path,
            backup_path,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_file = PathBuf::from("config.json");
        if config_file.exists() {
            let content = fs::read_to_string(config_file)?;
            let config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_file = PathBuf::from("config.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_file, content)?;
        Ok(())
    }
}

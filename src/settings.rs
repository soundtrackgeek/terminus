use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub model: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model: "gpt-4o-mini".to_string(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let path = Self::settings_path()?;
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::settings_path()?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    fn settings_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("terminus");
        path.push("settings.json");
        Ok(path)
    }
}

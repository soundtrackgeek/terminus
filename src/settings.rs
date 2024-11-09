use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub model: String,
    pub use_memory: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model: "gpt-4o-mini".to_string(),
            use_memory: true,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let path = Self::settings_path()?;
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            // Try to parse with current schema, if fails, attempt migration
            match serde_json::from_str(&contents) {
                Ok(settings) => Ok(settings),
                Err(_) => {
                    // Parse as legacy format (just model field)
                    #[derive(Deserialize)]
                    struct LegacySettings {
                        model: String,
                    }
                    let legacy: LegacySettings = serde_json::from_str(&contents)?;
                    let settings = Self {
                        model: legacy.model,
                        use_memory: true, // default value for migrated settings
                    };
                    settings.save()?; // Save with new schema
                    Ok(settings)
                }
            }
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

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct SystemMessage;

impl SystemMessage {
    pub fn load() -> Result<String> {
        let path = Self::message_path()?;
        if path.exists() {
            Ok(fs::read_to_string(path)?)
        } else {
            Ok(String::new())
        }
    }

    pub fn save(message: &str) -> Result<()> {
        let path = Self::message_path()?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, message)?;
        Ok(())
    }

    fn message_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("terminus");
        path.push("system_message.txt");
        Ok(path)
    }
}

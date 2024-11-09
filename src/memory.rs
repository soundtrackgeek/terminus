use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Memory;

impl Memory {
    pub fn load() -> Result<String> {
        let path = Self::memory_path()?;
        if path.exists() {
            Ok(fs::read_to_string(path)?)
        } else {
            Ok(String::new())
        }
    }

    pub fn save(content: &str) -> Result<()> {
        let path = Self::memory_path()?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn append(content: &str) -> Result<()> {
        let mut current = Self::load()?;
        if !current.is_empty() {
            current.push_str("\n\n");
        }
        current.push_str(content);
        Self::save(&current)
    }

    fn memory_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("terminus");
        path.push("memory.txt");
        Ok(path)
    }
}

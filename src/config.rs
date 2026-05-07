use std::path::PathBuf;
use std::fs;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub supabase_token: String,
}

impl Config {
    fn config_file_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "supa", "supa-admin")
            .context("No se pudo determinar el directorio de configuración del sistema")?;
        let config_dir = proj_dirs.config_dir();
        
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context("No se pudo crear el directorio de configuración")?;
        }
        
        Ok(config_dir.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_file_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_file_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

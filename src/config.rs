use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use zeroize::Zeroize;
use crate::utils::{config_dir, decrypt, derive_key, encrypt};

const CONFIG_FILENAME: &str = "config.enc";
const SALT_FILENAME: &str = "salt.bin";
const DEFAULT_PASSWORD: &[u8] = b"hazeveil-default-local-key-v1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HazeVeilConfig {
    pub level: u8,
    pub pattern: String,
    pub excluded_apps: Vec<String>,
    pub training_enabled: bool,
    pub context_aware: bool,
}

impl Default for HazeVeilConfig {
    fn default() -> Self {
        Self {
            level: 3,
            pattern: "random".to_string(),
            excluded_apps: Vec::new(),
            training_enabled: false,
            context_aware: true,
        }
    }
}

impl Drop for HazeVeilConfig {
    fn drop(&mut self) {
        self.excluded_apps.iter_mut().for_each(|s| s.zeroize());
    }
}

pub fn load() -> Result<HazeVeilConfig> {
    let cfg_path = config_path()?;
    if !cfg_path.exists() { return Ok(HazeVeilConfig::default()); }
    let salt = load_salt()?;
    let mut key = derive_key(DEFAULT_PASSWORD, &salt)?;
    let encrypted = std::fs::read(&cfg_path).context("Failed to read config")?;
    let plaintext = decrypt(&key, &encrypted).context("Failed to decrypt config")?;
    key.zeroize();
    Ok(serde_json::from_slice(&plaintext).context("Failed to parse config")?)
}

pub fn save(cfg: &HazeVeilConfig) -> Result<()> {
    let salt = load_or_create_salt()?;
    let mut key = derive_key(DEFAULT_PASSWORD, &salt)?;
    let plaintext = serde_json::to_vec(cfg)?;
    let encrypted = encrypt(&key, &plaintext)?;
    key.zeroize();
    std::fs::write(config_path()?, &encrypted)?;
    Ok(())
}

pub fn is_configured() -> bool {
    config_dir().map(|d| d.join(".configured").exists()).unwrap_or(false)
}

pub fn mark_configured() -> Result<()> {
    let path = config_dir()?.join(".configured");
    std::fs::write(path, "1")?;
    Ok(())
}

pub fn interactive_config() -> Result<()> {
    let cfg = load()?;
    println!("HazeVeil Configuration");
    println!("Level: {} | Pattern: {} | Excluded: {:?}", cfg.level, cfg.pattern, cfg.excluded_apps);
    save(&cfg)?;
    println!("Configuration saved.");
    Ok(())
}

fn config_path() -> Result<PathBuf> { Ok(config_dir()?.join(CONFIG_FILENAME)) }
fn salt_path() -> Result<PathBuf> { Ok(config_dir()?.join(SALT_FILENAME)) }

fn load_salt() -> Result<[u8; 16]> {
    let data = std::fs::read(salt_path()?)?;
    if data.len() < 16 { anyhow::bail!("Salt too short"); }
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&data[..16]);
    Ok(salt)
}

fn load_or_create_salt() -> Result<[u8; 16]> {
    let path = salt_path()?;
    if path.exists() { return load_salt(); }
    use rand::RngCore;
    let mut salt = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    std::fs::write(&path, &salt)?;
    Ok(salt)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_default() {
        let cfg = HazeVeilConfig::default();
        assert_eq!(cfg.level, 3);
        assert_eq!(cfg.pattern, "random");
        assert!(cfg.excluded_apps.is_empty());
    }
    #[test]
    fn test_config_save_load() {
        let cfg = HazeVeilConfig {
            level: 2,
            pattern: "developer".to_string(),
            excluded_apps: vec![],
            training_enabled: false,
            context_aware: true,
        };
        save(&cfg).unwrap();
        let loaded = load().unwrap();
        assert_eq!(loaded.level, 2);
        assert_eq!(loaded.pattern, "developer");
    }
}
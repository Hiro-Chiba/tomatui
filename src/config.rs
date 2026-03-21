use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::constants::APP_NAME;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub work_minutes: u64,
    pub break_minutes: u64,
    pub long_break_minutes: u64,
    pub sessions: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: 25,
            break_minutes: 5,
            long_break_minutes: 15,
            sessions: 4,
        }
    }
}

fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME);
    fs::create_dir_all(&config_dir).ok();
    config_dir.join("config.json")
}

pub fn load_config() -> Config {
    let path = config_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let config = Config::default();
        assert_eq!(config.work_minutes, 25);
        assert_eq!(config.break_minutes, 5);
        assert_eq!(config.long_break_minutes, 15);
        assert_eq!(config.sessions, 4);
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let config = Config {
            work_minutes: 30,
            break_minutes: 10,
            long_break_minutes: 20,
            sessions: 6,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.work_minutes, config.work_minutes);
        assert_eq!(restored.break_minutes, config.break_minutes);
        assert_eq!(restored.long_break_minutes, config.long_break_minutes);
        assert_eq!(restored.sessions, config.sessions);
    }

    #[test]
    fn invalid_json_falls_back_to_default() {
        let result: Config = serde_json::from_str("not json").unwrap_or_default();
        let default = Config::default();
        assert_eq!(result.work_minutes, default.work_minutes);
        assert_eq!(result.break_minutes, default.break_minutes);
        assert_eq!(result.long_break_minutes, default.long_break_minutes);
        assert_eq!(result.sessions, default.sessions);
    }
}

pub fn print_config() {
    let config = load_config();
    let path = config_path();
    println!("\n  Tomatui Settings ({})", path.display());
    println!("  {}", "-".repeat(40));
    println!("  work           {} min", config.work_minutes);
    println!("  break          {} min", config.break_minutes);
    println!("  long_break     {} min", config.long_break_minutes);
    println!("  sessions       {}", config.sessions);
    println!();
}

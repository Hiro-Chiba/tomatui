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

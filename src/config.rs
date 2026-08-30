use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::{APP_DISPLAY_NAME, APP_NAME, SECONDS_PER_MINUTE};
use crate::timer::TimerConfig;

const CONFIG_FILE_NAME: &str = "config.json";
const DEFAULT_WORK_MINUTES: u64 = 25;
const DEFAULT_BREAK_MINUTES: u64 = 5;
const DEFAULT_LONG_BREAK_MINUTES: u64 = 15;
const DEFAULT_SESSIONS: u32 = 4;
const SETTINGS_SEPARATOR_WIDTH: usize = 40;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub work_minutes: u64,
    pub break_minutes: u64,
    pub long_break_minutes: u64,
    pub sessions: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: DEFAULT_WORK_MINUTES,
            break_minutes: DEFAULT_BREAK_MINUTES,
            long_break_minutes: DEFAULT_LONG_BREAK_MINUTES,
            sessions: DEFAULT_SESSIONS,
        }
    }
}

impl Config {
    fn validate(&self) -> io::Result<()> {
        for (name, minutes) in [
            ("work", self.work_minutes),
            ("break", self.break_minutes),
            ("long break", self.long_break_minutes),
        ] {
            if minutes == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{name} duration must be at least 1 minute"),
                ));
            }
            if minutes.checked_mul(SECONDS_PER_MINUTE).is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{name} duration is too large"),
                ));
            }
        }

        if self.sessions == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "sessions must be at least 1",
            ));
        }

        Ok(())
    }

    pub fn timer_config(&self) -> io::Result<TimerConfig> {
        self.validate()?;
        Ok(TimerConfig {
            work_secs: self.work_minutes * SECONDS_PER_MINUTE,
            break_secs: self.break_minutes * SECONDS_PER_MINUTE,
            long_break_secs: self.long_break_minutes * SECONDS_PER_MINUTE,
            sessions: self.sessions,
        })
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME)
        .join(CONFIG_FILE_NAME)
}

fn load_config_from(path: &Path) -> Result<Config, Box<dyn Error>> {
    let config = match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => Config::default(),
        Err(error) => return Err(error.into()),
    };
    config.validate()?;
    Ok(config)
}

fn save_config_to(path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    config.validate()?;
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    load_config_from(&config_path())
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    save_config_to(&config_path(), config)
}

pub fn print_config() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let path = config_path();
    println!("\n  {APP_DISPLAY_NAME} Settings ({})", path.display());
    println!("  {}", "-".repeat(SETTINGS_SEPARATOR_WIDTH));
    println!("  work           {} min", config.work_minutes);
    println!("  break          {} min", config.break_minutes);
    println!("  long_break     {} min", config.long_break_minutes);
    println!("  sessions       {}", config.sessions);
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            Self(
                std::env::temp_dir()
                    .join(format!("tomatui-config-test-{}-{id}", std::process::id())),
            )
        }

        fn file(&self) -> PathBuf {
            self.0.join("config.json")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn default_values_are_valid() {
        let config = Config::default();
        assert_eq!(config.work_minutes, 25);
        assert_eq!(config.break_minutes, 5);
        assert_eq!(config.long_break_minutes, 15);
        assert_eq!(config.sessions, 4);
        assert!(config.timer_config().is_ok());
    }

    #[test]
    fn missing_file_uses_defaults_without_creating_directories() {
        let dir = TestDir::new();
        assert_eq!(load_config_from(&dir.file()).unwrap(), Config::default());
        assert!(!dir.0.exists());
    }

    #[test]
    fn save_and_load_roundtrip_uses_isolated_path() {
        let dir = TestDir::new();
        let config = Config {
            work_minutes: 30,
            break_minutes: 10,
            long_break_minutes: 20,
            sessions: 6,
        };

        save_config_to(&dir.file(), &config).unwrap();
        assert_eq!(load_config_from(&dir.file()).unwrap(), config);
    }

    #[test]
    fn missing_fields_use_defaults() {
        let config: Config = serde_json::from_str(r#"{"work_minutes":30}"#).unwrap();
        assert_eq!(config.work_minutes, 30);
        assert_eq!(config.break_minutes, 5);
        assert_eq!(config.long_break_minutes, 15);
        assert_eq!(config.sessions, 4);
    }

    #[test]
    fn malformed_file_returns_an_error() {
        let dir = TestDir::new();
        fs::create_dir_all(&dir.0).unwrap();
        fs::write(dir.file(), "not json").unwrap();
        assert!(load_config_from(&dir.file()).is_err());
    }

    #[test]
    fn invalid_values_are_rejected() {
        let config = Config {
            work_minutes: 0,
            ..Config::default()
        };
        assert!(config.timer_config().is_err());

        let config = Config {
            sessions: 0,
            ..Config::default()
        };
        assert!(config.timer_config().is_err());

        let config = Config {
            work_minutes: u64::MAX,
            ..Config::default()
        };
        assert!(config.timer_config().is_err());
    }

    #[test]
    fn save_reports_parent_directory_errors() {
        let dir = TestDir::new();
        fs::create_dir_all(&dir.0).unwrap();
        let blocking_file = dir.0.join("not-a-directory");
        fs::write(&blocking_file, "file").unwrap();

        assert!(save_config_to(&blocking_file.join("config.json"), &Config::default()).is_err());
    }
}

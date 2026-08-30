use clap::{Parser, Subcommand};

use crate::constants::SECONDS_PER_MINUTE;

const DEFAULT_HISTORY_DAYS: u32 = 7;
const MAX_HISTORY_DAYS: u32 = 3660;
const POSITIVE_WHOLE_NUMBER_ERROR: &str = "must be a positive whole number";
const MINIMUM_VALUE_ERROR: &str = "must be at least 1";
const VALUE_TOO_LARGE_ERROR: &str = "is too large";

fn parse_minutes(value: &str) -> Result<u64, String> {
    let minutes = value
        .parse::<u64>()
        .map_err(|_| POSITIVE_WHOLE_NUMBER_ERROR.to_string())?;
    if minutes == 0 {
        return Err(MINIMUM_VALUE_ERROR.to_string());
    }
    if minutes.checked_mul(SECONDS_PER_MINUTE).is_none() {
        return Err(VALUE_TOO_LARGE_ERROR.to_string());
    }
    Ok(minutes)
}

fn parse_positive_u32(value: &str) -> Result<u32, String> {
    let number = value
        .parse::<u32>()
        .map_err(|_| POSITIVE_WHOLE_NUMBER_ERROR.to_string())?;
    if number == 0 {
        return Err(MINIMUM_VALUE_ERROR.to_string());
    }
    Ok(number)
}

fn parse_history_days(value: &str) -> Result<u32, String> {
    let days = parse_positive_u32(value)?;
    if days > MAX_HISTORY_DAYS {
        return Err(format!("must be at most {MAX_HISTORY_DAYS}"));
    }
    Ok(days)
}

#[derive(Parser)]
#[command(version, about = "Terminal Pomodoro Timer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a pomodoro timer
    Start {
        /// Minimal one-line mode
        #[arg(short, long)]
        minimal: bool,

        /// Work duration in minutes (overrides config)
        #[arg(short, long, value_parser = parse_minutes)]
        work: Option<u64>,

        /// Break duration in minutes (overrides config)
        #[arg(short, long, value_parser = parse_minutes)]
        r#break: Option<u64>,

        /// Long break duration in minutes (overrides config)
        #[arg(short, long, value_parser = parse_minutes)]
        long_break: Option<u64>,

        /// Number of sessions before long break (overrides config)
        #[arg(short, long, value_parser = parse_positive_u32)]
        sessions: Option<u32>,
    },
    /// Show pomodoro statistics
    Stats {
        #[command(subcommand)]
        command: Option<StatsCommands>,
    },
    /// View or update settings
    Config {
        /// Set work duration in minutes
        #[arg(short, long, value_parser = parse_minutes)]
        work: Option<u64>,

        /// Set break duration in minutes
        #[arg(short, long, value_parser = parse_minutes)]
        r#break: Option<u64>,

        /// Set long break duration in minutes
        #[arg(short, long, value_parser = parse_minutes)]
        long_break: Option<u64>,

        /// Set number of sessions before long break
        #[arg(short, long, value_parser = parse_positive_u32)]
        sessions: Option<u32>,

        /// Reset to default settings
        #[arg(long)]
        reset: bool,
    },
}

#[derive(Subcommand)]
pub enum StatsCommands {
    /// Show today's statistics
    Today,
    /// Show daily history
    History {
        /// Number of days to show
        #[arg(short, long, default_value_t = DEFAULT_HISTORY_DAYS, value_parser = parse_history_days)]
        days: u32,
    },
    /// Show all-time summary
    Summary,
    /// Clear all statistics
    Clear,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_durations_and_sessions() {
        assert!(Cli::try_parse_from(["tomatui", "start", "--work", "0"]).is_err());
        assert!(Cli::try_parse_from(["tomatui", "start", "--sessions", "0"]).is_err());
        assert!(Cli::try_parse_from(["tomatui", "config", "--break", "0"]).is_err());
    }

    #[test]
    fn accepts_single_minute_and_session() {
        assert!(
            Cli::try_parse_from(["tomatui", "start", "--work", "1", "--sessions", "1",]).is_ok()
        );
    }

    #[test]
    fn rejects_duration_that_cannot_convert_to_seconds() {
        assert!(
            Cli::try_parse_from(["tomatui", "start", "--work", &u64::MAX.to_string(),]).is_err()
        );
    }

    #[test]
    fn history_days_use_safe_boundaries() {
        for days in [1, MAX_HISTORY_DAYS] {
            assert!(
                Cli::try_parse_from(["tomatui", "stats", "history", "--days", &days.to_string(),])
                    .is_ok()
            );
        }

        for days in [MAX_HISTORY_DAYS + 1, u32::MAX] {
            assert!(
                Cli::try_parse_from(["tomatui", "stats", "history", "--days", &days.to_string(),])
                    .is_err()
            );
        }
    }

    #[test]
    fn history_defaults_to_seven_days() {
        let cli = Cli::try_parse_from(["tomatui", "stats", "history"]).unwrap();
        let Commands::Stats {
            command: Some(StatsCommands::History { days }),
        } = cli.command
        else {
            panic!("expected the history command");
        };

        assert_eq!(days, 7);
    }
}

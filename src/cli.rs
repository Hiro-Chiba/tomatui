use clap::{Parser, Subcommand};

fn parse_minutes(value: &str) -> Result<u64, String> {
    let minutes = value
        .parse::<u64>()
        .map_err(|_| "must be a positive whole number".to_string())?;
    if minutes == 0 {
        return Err("must be at least 1".to_string());
    }
    if minutes.checked_mul(60).is_none() {
        return Err("is too large".to_string());
    }
    Ok(minutes)
}

fn parse_positive_u32(value: &str) -> Result<u32, String> {
    let number = value
        .parse::<u32>()
        .map_err(|_| "must be a positive whole number".to_string())?;
    if number == 0 {
        return Err("must be at least 1".to_string());
    }
    Ok(number)
}

#[derive(Parser)]
#[command(name = "tomatui", version, about = "Terminal Pomodoro Timer")]
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
        #[arg(short, long, default_value_t = 7, value_parser = parse_positive_u32)]
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
}

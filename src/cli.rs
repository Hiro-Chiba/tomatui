use clap::{Args, Parser, Subcommand};

use crate::config::OnEnd;
use crate::constants::SECONDS_PER_MINUTE;

const DEFAULT_HISTORY_DAYS: u32 = 7;
const MAX_HISTORY_DAYS: u32 = 3660;
const POSITIVE_WHOLE_NUMBER_ERROR: &str = "must be a positive whole number";
const MINIMUM_VALUE_ERROR: &str = "must be at least 1";
const VALUE_TOO_LARGE_ERROR: &str = "is too large";

fn parse_minutes(value: &str) -> Result<u64, String> {
    let (number, multiplier) = if let Some(hours) = value.strip_suffix('h') {
        (hours, 60)
    } else {
        (value.strip_suffix('m').unwrap_or(value), 1)
    };
    let minutes = number
        .parse::<u64>()
        .map_err(|_| "use whole minutes or hours, such as 30, 30m, or 1h".to_string())?
        .checked_mul(multiplier)
        .ok_or_else(|| VALUE_TOO_LARGE_ERROR.to_string())?;
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
#[command(
    version,
    about = "Terminal Pomodoro Timer",
    args_conflicts_with_subcommands = true,
    after_help = "Examples:\n  tomatui             Start with saved settings\n  tomatui 30m         Work for 30 minutes\n  tomatui 45m 10m     Work for 45 minutes, then rest for 10\n  tomatui -m          Use one line\n  tomatui 1h -m       Work for one hour in one-line mode\n  tomatui config -w 30m -b 10m   Save your usual durations"
)]
pub struct Cli {
    #[command(flatten)]
    pub start: StartArgs,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn into_command(self) -> Commands {
        match self.command.unwrap_or(Commands::Start(self.start)) {
            Commands::Start(mut args) => {
                args.work = args.work.or(args.work_duration.take());
                args.r#break = args.r#break.or(args.break_duration.take());
                Commands::Start(args)
            }
            command => command,
        }
    }
}

#[derive(Args, Debug, Default, PartialEq)]
pub struct StartArgs {
    /// Work duration, for example 30m or 1h (plain numbers mean minutes)
    #[arg(value_name = "WORK", value_parser = parse_minutes, conflicts_with = "work")]
    pub work_duration: Option<u64>,

    /// Break duration, for example 10m (overrides config for this run)
    #[arg(value_name = "BREAK", value_parser = parse_minutes, requires = "work_duration", conflicts_with = "break")]
    pub break_duration: Option<u64>,

    /// Minimal one-line mode
    #[arg(short, long)]
    pub minimal: bool,

    /// Work duration, e.g. 30m or 1h (overrides config)
    #[arg(short, long, value_parser = parse_minutes)]
    pub work: Option<u64>,

    /// Break duration, e.g. 30m or 1h (overrides config)
    #[arg(short, long, value_parser = parse_minutes)]
    pub r#break: Option<u64>,

    /// Long break duration, e.g. 30m or 1h (overrides config)
    #[arg(short, long, value_parser = parse_minutes)]
    pub long_break: Option<u64>,

    /// Number of sessions before long break (overrides config)
    #[arg(short, long, value_parser = parse_positive_u32)]
    pub sessions: Option<u32>,

    /// Action after each completed phase: ask, start the next, or quit
    #[arg(long, value_enum)]
    pub on_end: Option<OnEnd>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a pomodoro timer (also the default without a command)
    Start(StartArgs),
    /// Show pomodoro statistics
    Stats {
        #[command(subcommand)]
        command: Option<StatsCommands>,
    },
    /// View or update settings
    Config {
        /// Set work duration, e.g. 30m or 1h
        #[arg(short, long, value_parser = parse_minutes)]
        work: Option<u64>,

        /// Set break duration, e.g. 10m
        #[arg(short, long, value_parser = parse_minutes)]
        r#break: Option<u64>,

        /// Set long break duration, e.g. 15m
        #[arg(short, long, value_parser = parse_minutes)]
        long_break: Option<u64>,

        /// Set number of sessions before long break
        #[arg(short, long, value_parser = parse_positive_u32)]
        sessions: Option<u32>,

        /// Action after each completed phase: ask, start the next, or quit
        #[arg(long, value_enum)]
        on_end: Option<OnEnd>,

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
    fn short_durations_apply_to_both_launch_forms() {
        for prefix in [vec!["tomatui"], vec!["tomatui", "start"]] {
            let cli = Cli::try_parse_from(prefix.into_iter().chain(["1h", "10m", "-m"])).unwrap();
            let Commands::Start(args) = cli.into_command() else {
                panic!("expected timer")
            };
            assert_eq!(args.work, Some(60));
            assert_eq!(args.r#break, Some(10));
            assert!(args.minimal);
        }
        let cli = Cli::try_parse_from(["tomatui", "30"]).unwrap();
        let Commands::Start(args) = cli.into_command() else {
            panic!("expected timer")
        };
        assert_eq!(args.work, Some(30));
        assert_eq!(args.r#break, None);
        assert!(Cli::try_parse_from(["tomatui", "config", "-w", "1h", "-b", "10m"]).is_ok());
    }

    #[test]
    fn rejects_ambiguous_or_invalid_short_durations() {
        for args in [
            vec!["30m", "-w", "20"],
            vec!["30m", "10m", "-b", "5"],
            vec!["30m", "10m", "5m"],
            vec!["30m", "stats"],
        ] {
            assert!(Cli::try_parse_from(["tomatui"].into_iter().chain(args)).is_err());
        }
        for value in [
            "0",
            "0m",
            "0h",
            "-1",
            "1.5h",
            "30s",
            "m",
            "1h30m",
            "18446744073709551615h",
            "18446744073709551615m",
        ] {
            assert!(Cli::try_parse_from(["tomatui", value]).is_err(), "{value}");
        }
        assert_eq!(parse_minutes("1h").unwrap(), 60);
        assert_eq!(parse_minutes("30m").unwrap(), 30);
    }

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
        } = cli.into_command()
        else {
            panic!("expected the history command");
        };

        assert_eq!(days, 7);
    }
    #[test]
    fn accepts_end_actions_for_start_and_persistent_config() {
        for command in ["start", "config"] {
            for mode in ["ask", "start", "quit"] {
                assert!(Cli::try_parse_from(["tomatui", command, "--on-end", mode]).is_ok());
            }
            assert!(Cli::try_parse_from(["tomatui", command, "--on-end", "invalid"]).is_err());
        }
        let cli = Cli::try_parse_from(["tomatui", "start", "--on-end", "ask"]).unwrap();
        assert!(matches!(
            cli.into_command(),
            Commands::Start(StartArgs {
                on_end: Some(OnEnd::Ask),
                ..
            })
        ));
    }
    #[test]
    fn no_arguments_starts_full_timer_with_saved_settings() {
        let cli = Cli::try_parse_from(["tomatui"]).unwrap();
        let Commands::Start(args) = cli.into_command() else {
            panic!("expected default start command");
        };
        assert_eq!(args, StartArgs::default());
    }

    #[test]
    fn minimal_mode_needs_only_one_flag() {
        let cli = Cli::try_parse_from(["tomatui", "-m"]).unwrap();
        let Commands::Start(args) = cli.into_command() else {
            panic!("expected default start command");
        };
        assert_eq!(
            args,
            StartArgs {
                minimal: true,
                ..StartArgs::default()
            }
        );
    }

    #[test]
    fn root_overrides_and_existing_start_syntax_are_equivalent() {
        let flags = [
            "-m", "-w", "30", "-b", "10", "-l", "20", "-s", "3", "--on-end", "ask",
        ];
        for prefix in [vec!["tomatui"], vec!["tomatui", "start"]] {
            let cli = Cli::try_parse_from(prefix.into_iter().chain(flags)).unwrap();
            let Commands::Start(args) = cli.into_command() else {
                panic!("expected start command");
            };
            assert_eq!(
                args,
                StartArgs {
                    minimal: true,
                    work: Some(30),
                    r#break: Some(10),
                    long_break: Some(20),
                    sessions: Some(3),
                    on_end: Some(OnEnd::Ask),
                    ..StartArgs::default()
                }
            );
        }
    }

    #[test]
    fn root_timer_options_cannot_be_silently_ignored_by_subcommands() {
        for command in ["start", "stats", "config"] {
            for flags in [vec!["-m"], vec!["-w", "30"], vec!["--on-end", "ask"]] {
                assert!(
                    Cli::try_parse_from(["tomatui"].into_iter().chain(flags).chain([command]))
                        .is_err()
                );
            }
        }
        assert!(Cli::try_parse_from(["tomatui", "stats", "-m"]).is_err());
        assert!(Cli::try_parse_from(["tomatui", "config", "-m"]).is_err());
        assert!(Cli::try_parse_from(["tomatui", "config", "-w", "30"]).is_ok());
        assert!(Cli::try_parse_from(["tomatui", "--work", "0"]).is_err());
    }
}

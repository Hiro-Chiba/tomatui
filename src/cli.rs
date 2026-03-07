use clap::{Parser, Subcommand};

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
        #[arg(short, long)]
        work: Option<u64>,

        /// Break duration in minutes (overrides config)
        #[arg(short, long)]
        r#break: Option<u64>,

        /// Long break duration in minutes (overrides config)
        #[arg(short, long)]
        long_break: Option<u64>,

        /// Number of sessions before long break (overrides config)
        #[arg(short, long)]
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
        #[arg(short, long)]
        work: Option<u64>,

        /// Set break duration in minutes
        #[arg(short, long)]
        r#break: Option<u64>,

        /// Set long break duration in minutes
        #[arg(short, long)]
        long_break: Option<u64>,

        /// Set number of sessions before long break
        #[arg(short, long)]
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
        #[arg(short, long, default_value_t = 7)]
        days: u32,
    },
    /// Show all-time summary
    Summary,
    /// Clear all statistics
    Clear,
}

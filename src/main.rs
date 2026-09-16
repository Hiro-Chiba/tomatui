mod app;
mod cli;
mod config;
mod constants;
mod minimal;
mod notification;
mod stats;
mod timer;
mod tui;
mod ui;

use clap::Parser;
use cli::{Cli, Commands, StartArgs, StatsCommands};
use config::{Config, load_config, save_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.into_command() {
        Commands::Start(StartArgs {
            minimal,
            work,
            r#break,
            long_break,
            sessions,
            on_end,
            ..
        }) => {
            let mut cfg = load_config()?;
            cfg.work_minutes = work.unwrap_or(cfg.work_minutes);
            cfg.break_minutes = r#break.unwrap_or(cfg.break_minutes);
            cfg.long_break_minutes = long_break.unwrap_or(cfg.long_break_minutes);
            cfg.sessions = sessions.unwrap_or(cfg.sessions);
            cfg.on_end = on_end.unwrap_or(cfg.on_end);
            let config = cfg.timer_config()?;

            if minimal {
                minimal::run(config)?;
            } else {
                tui::run(config)?;
            }
        }
        Commands::Stats { command } => match command {
            Some(StatsCommands::Today) => stats::print_today()?,
            Some(StatsCommands::History { days }) => stats::print_history(days)?,
            Some(StatsCommands::Summary) => stats::print_summary()?,
            Some(StatsCommands::Clear) => stats::clear_stats()?,
            None => stats::print_today()?,
        },
        Commands::Config {
            work,
            r#break,
            long_break,
            sessions,
            on_end,
            reset,
        } => {
            if reset {
                save_config(&Config::default())?;
                println!("  Settings reset to defaults.");
                config::print_config()?;
                return Ok(());
            }

            let has_updates = work.is_some()
                || r#break.is_some()
                || long_break.is_some()
                || sessions.is_some()
                || on_end.is_some();

            if has_updates {
                let mut cfg = load_config()?;
                if let Some(v) = work {
                    cfg.work_minutes = v;
                }
                if let Some(v) = r#break {
                    cfg.break_minutes = v;
                }
                if let Some(v) = long_break {
                    cfg.long_break_minutes = v;
                }
                if let Some(v) = sessions {
                    cfg.sessions = v;
                }
                if let Some(v) = on_end {
                    cfg.on_end = v;
                }
                save_config(&cfg)?;
                println!("  Settings updated.");
            }

            config::print_config()?;
        }
    }

    Ok(())
}

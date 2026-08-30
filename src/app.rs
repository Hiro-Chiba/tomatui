use std::time::Instant;

use crate::constants::{
    APP_DISPLAY_NAME, BREAK_KEY, DATE_FORMAT, PAUSE_KEY, QUIT_KEY, SECONDS_PER_MINUTE, SKIP_KEY,
    SPACE_KEY, WORK_KEY,
};
use crate::notification::{bell, notify};
use crate::stats::record_pomodoro;
use crate::timer::{Phase, Timer, TimerConfig};

const STATS_CACHE_SECS: u64 = 30;
const WORK_COMPLETE_MESSAGE: &str = "Work session complete! Time for a break.";
const WORK_SKIPPED_MESSAGE: &str = "Work session skipped.";
const BREAK_COMPLETE_MESSAGE: &str = "Break is over! Time to work.";

pub struct App {
    pub timer: Timer,
    pub should_quit: bool,
    cached_stats: (u32, u64),
    stats_updated_at: Instant,
}

impl App {
    pub fn new(config: TimerConfig) -> Self {
        Self::with_cached_stats(config, Self::fetch_today_stats())
    }

    fn with_cached_stats(config: TimerConfig, cached_stats: (u32, u64)) -> Self {
        Self {
            timer: Timer::new(config),
            should_quit: false,
            cached_stats,
            stats_updated_at: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        let previous_phase = self.timer.phase;
        let previous_display_seconds = self.timer.display_seconds();
        let phase_complete = self.timer.tick();
        if phase_complete {
            self.on_phase_complete();
        }

        self.timer.phase != previous_phase
            || self.timer.display_seconds() != previous_display_seconds
    }

    fn on_phase_complete(&mut self) {
        bell();

        if self.timer.phase == Phase::Work && !self.timer.skipped {
            let work_minutes = self.timer.config.work_secs / SECONDS_PER_MINUTE;
            match record_pomodoro(work_minutes) {
                Ok(today_stats) => {
                    self.cached_stats = today_stats;
                    self.stats_updated_at = Instant::now();
                }
                Err(e) => {
                    eprintln!("Failed to record pomodoro: {}", e);
                }
            }
            notify(APP_DISPLAY_NAME, WORK_COMPLETE_MESSAGE);
        } else if self.timer.phase == Phase::Work {
            notify(APP_DISPLAY_NAME, WORK_SKIPPED_MESSAGE);
        } else {
            notify(APP_DISPLAY_NAME, BREAK_COMPLETE_MESSAGE);
        }

        self.timer.advance_phase();
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            QUIT_KEY => self.should_quit = true,
            PAUSE_KEY | SPACE_KEY => self.timer.toggle_pause(),
            SKIP_KEY => self.timer.skip(),
            WORK_KEY => self.timer.switch_to_work(),
            BREAK_KEY => self.timer.switch_to_break(),
            _ => {}
        }
    }

    fn fetch_today_stats() -> (u32, u64) {
        match crate::stats::load_stats() {
            Ok(stats) => {
                let today = chrono::Local::now().format(DATE_FORMAT).to_string();
                stats.get_day(&today)
            }
            Err(e) => {
                eprintln!("Failed to load statistics: {}", e);
                (0, 0)
            }
        }
    }

    pub fn today_stats(&mut self) -> (u32, u64) {
        if self.stats_updated_at.elapsed().as_secs() >= STATS_CACHE_SECS {
            self.cached_stats = Self::fetch_today_stats();
            self.stats_updated_at = Instant::now();
        }
        self.cached_stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> TimerConfig {
        TimerConfig {
            work_secs: 1500,
            break_secs: 300,
            long_break_secs: 900,
            sessions: 4,
        }
    }

    fn test_app() -> App {
        App::with_cached_stats(test_config(), (0, 0))
    }

    #[test]
    fn test_on_key_quit() {
        let mut app = test_app();
        app.on_key('q');
        assert!(app.should_quit);
    }

    #[test]
    fn test_on_key_pause_toggle() {
        let mut app = test_app();
        assert!(!app.timer.paused);
        app.on_key('p');
        assert!(app.timer.paused);
        app.on_key(' ');
        assert!(!app.timer.paused);
    }

    #[test]
    fn test_on_key_switch_work() {
        let mut app = test_app();
        app.on_key('b');
        assert!(app.timer.phase == Phase::Break || app.timer.phase == Phase::LongBreak);
        app.on_key('w');
        assert_eq!(app.timer.phase, Phase::Work);
    }

    #[test]
    fn test_skip_sets_remaining_zero() {
        let mut app = test_app();
        app.on_key('s');
        assert!(app.timer.remaining.is_zero());
        assert!(app.timer.skipped);
    }

    #[test]
    fn test_skip_does_not_record_pomodoro() {
        let mut app = test_app();
        let initial_pomos = app.cached_stats.0;
        app.on_key('s');
        // Simulate tick detecting phase complete
        assert!(app.tick());
        // cached_stats should not have incremented
        assert_eq!(app.cached_stats.0, initial_pomos);
    }

    #[test]
    fn test_today_stats_uses_cache() {
        let mut app = test_app();
        let stats1 = app.today_stats();
        let stats2 = app.today_stats();
        assert_eq!(stats1, stats2);
    }

    #[test]
    fn test_tick_returns_false_while_paused_without_display_change() {
        let mut app = test_app();
        app.timer.toggle_pause();

        assert!(!app.tick());
    }
}

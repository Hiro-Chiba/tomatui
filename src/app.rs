use std::time::Instant;

use crate::constants::{DATE_FORMAT, SECS_PER_MIN};
use crate::notification::{bell, notify};
use crate::stats::record_pomodoro;
use crate::timer::{Phase, Timer, TimerConfig};

const STATS_CACHE_SECS: u64 = 30;

pub struct App {
    pub timer: Timer,
    pub should_quit: bool,
    cached_stats: (u32, u64),
    stats_updated_at: Instant,
}

impl App {
    pub fn new(config: TimerConfig) -> Self {
        let stats = Self::fetch_today_stats();
        Self {
            timer: Timer::new(config),
            should_quit: false,
            cached_stats: stats,
            stats_updated_at: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        let phase_complete = self.timer.tick();
        if phase_complete {
            self.on_phase_complete();
        }
    }

    fn on_phase_complete(&mut self) {
        bell();

        if self.timer.phase == Phase::Work && !self.timer.skipped {
            let work_minutes = self.timer.config.work_secs / SECS_PER_MIN;
            match record_pomodoro(work_minutes) {
                Ok(()) => {
                    self.cached_stats.0 += 1;
                    self.cached_stats.1 += work_minutes;
                    self.stats_updated_at = Instant::now();
                }
                Err(e) => {
                    eprintln!("Failed to record pomodoro: {}", e);
                }
            }
            notify("Pomo", "Work session complete! Time for a break.");
        } else if self.timer.phase == Phase::Work {
            notify("Pomo", "Work session skipped.");
        } else {
            notify("Pomo", "Break is over! Time to work.");
        }

        self.timer.advance_phase();
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            'q' => self.should_quit = true,
            'p' | ' ' => self.timer.toggle_pause(),
            's' => self.timer.skip(),
            'w' => self.timer.switch_to_work(),
            'b' => self.timer.switch_to_break(),
            _ => {}
        }
    }

    fn fetch_today_stats() -> (u32, u64) {
        let stats = crate::stats::load_stats();
        let today = chrono::Local::now().format(DATE_FORMAT).to_string();
        stats.get_day(&today)
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

    #[test]
    fn test_on_key_quit() {
        let mut app = App::new(test_config());
        app.on_key('q');
        assert!(app.should_quit);
    }

    #[test]
    fn test_on_key_pause_toggle() {
        let mut app = App::new(test_config());
        assert!(!app.timer.paused);
        app.on_key('p');
        assert!(app.timer.paused);
        app.on_key(' ');
        assert!(!app.timer.paused);
    }

    #[test]
    fn test_on_key_switch_work() {
        let mut app = App::new(test_config());
        app.on_key('b');
        assert!(app.timer.phase == Phase::Break || app.timer.phase == Phase::LongBreak);
        app.on_key('w');
        assert_eq!(app.timer.phase, Phase::Work);
    }

    #[test]
    fn test_on_key_unknown_does_nothing() {
        let mut app = App::new(test_config());
        let paused_before = app.timer.paused;
        let phase_before = app.timer.phase;
        app.on_key('x');
        assert_eq!(app.timer.paused, paused_before);
        assert_eq!(app.timer.phase, phase_before);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_skip_sets_remaining_zero() {
        let mut app = App::new(test_config());
        app.on_key('s');
        assert!(app.timer.remaining.is_zero());
        assert!(app.timer.skipped);
    }

    #[test]
    fn test_skip_does_not_record_pomodoro() {
        let mut app = App::new(test_config());
        let initial_pomos = app.cached_stats.0;
        app.on_key('s');
        // Simulate tick detecting phase complete
        app.tick();
        // cached_stats should not have incremented
        assert_eq!(app.cached_stats.0, initial_pomos);
    }

    #[test]
    fn test_today_stats_uses_cache() {
        let mut app = App::new(test_config());
        let stats1 = app.today_stats();
        let stats2 = app.today_stats();
        assert_eq!(stats1, stats2);
    }
}

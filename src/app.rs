use std::time::Instant;

use crate::config::OnEnd;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
    pub waiting_for_next: bool,
    pub status_message: Option<String>,
    completed_sessions: u32,
    completed_minutes: u64,
    unsaved_sessions: u32,
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
            waiting_for_next: false,
            status_message: None,
            completed_sessions: 0,
            completed_minutes: 0,
            unsaved_sessions: 0,
            cached_stats,
            stats_updated_at: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        self.tick_with(record_pomodoro, |message| {
            bell();
            notify(APP_DISPLAY_NAME, message);
        })
    }

    fn tick_with(
        &mut self,
        mut record: impl FnMut(u64) -> Result<(u32, u64), Box<dyn std::error::Error>>,
        mut announce: impl FnMut(&str),
    ) -> bool {
        if self.waiting_for_next || self.should_quit {
            return false;
        }
        let previous_display_seconds = self.timer.display_seconds();
        if !self.timer.tick() {
            return self.timer.display_seconds() != previous_display_seconds;
        }

        let skipped = self.timer.skipped;
        let message = if self.timer.phase == Phase::Work && !skipped {
            let work_minutes = self.timer.total.as_secs() / SECONDS_PER_MINUTE;
            self.completed_sessions = self.completed_sessions.saturating_add(1);
            self.completed_minutes = self.completed_minutes.saturating_add(work_minutes);
            match record(work_minutes) {
                Ok(today_stats) => {
                    self.cached_stats = today_stats;
                    self.stats_updated_at = Instant::now();
                }
                Err(error) => {
                    self.unsaved_sessions = self.unsaved_sessions.saturating_add(1);
                    self.status_message = Some(format!("Statistics not saved: {error}"));
                }
            }
            WORK_COMPLETE_MESSAGE
        } else if self.timer.phase == Phase::Work {
            WORK_SKIPPED_MESSAGE
        } else {
            BREAK_COMPLETE_MESSAGE
        };
        announce(message);

        if skipped {
            self.timer.advance_phase();
        } else {
            match self.timer.config.on_end {
                OnEnd::Ask => self.waiting_for_next = true,
                OnEnd::Start => self.timer.advance_phase(),
                OnEnd::Quit => self.should_quit = true,
            }
        }
        true
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }
        match key.code {
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Enter => self.on_key('\n'),
            KeyCode::Up => self.on_key('+'),
            KeyCode::Char(key) => self.on_key(key),
            _ => {}
        }
    }

    pub fn on_key(&mut self, key: char) {
        if self.should_quit {
            return;
        }
        if key == QUIT_KEY {
            self.should_quit = true;
            return;
        }
        if self.waiting_for_next {
            if matches!(key, '\n' | SKIP_KEY) {
                self.waiting_for_next = false;
                self.timer.advance_phase();
            }
            return;
        }
        match key {
            PAUSE_KEY | SPACE_KEY => self.timer.toggle_pause(),
            SKIP_KEY => self.timer.skip(),
            WORK_KEY => self.timer.switch_to_work(),
            BREAK_KEY => self.timer.switch_to_break(),
            '+' => {
                self.timer.add_minute();
            }
            _ => {}
        }
    }

    pub fn summary_text(&self) -> String {
        let sessions = self.completed_sessions;
        let noun = if sessions == 1 { "session" } else { "sessions" };
        let mut summary = format!(
            "Completed {sessions} focus {noun} · {} min",
            self.completed_minutes
        );
        if self.unsaved_sessions > 0 {
            summary.push_str(&format!(
                " · {} not saved to statistics",
                self.unsaved_sessions
            ));
        }
        summary
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
            on_end: OnEnd::Start,
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
        assert!(app.tick_with(|_| panic!("skip must not record"), |_| {}));
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
    #[test]
    fn ask_records_extended_work_once_and_waits_without_counting_wait_time() {
        let mut app = test_app();
        app.timer.config.on_end = OnEnd::Ask;
        app.on_key('+');
        app.timer.remaining = std::time::Duration::ZERO;
        let mut records = 0;
        let mut announcements = 0;
        assert!(app.tick_with(
            |minutes| {
                records += 1;
                assert_eq!(minutes, 26);
                Ok((1, minutes))
            },
            |_| announcements += 1
        ));
        assert!(app.waiting_for_next);
        assert_eq!(app.timer.phase, Phase::Work);
        assert!(!app.tick_with(
            |_| panic!("duplicate record"),
            |_| panic!("duplicate notification")
        ));
        for key in ['p', ' ', 'w', 'b', '+'] {
            app.on_key(key);
            assert!(app.waiting_for_next);
            assert!(app.timer.remaining.is_zero());
            assert!(!app.timer.paused);
        }
        app.on_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(!app.waiting_for_next);
        assert_eq!(app.timer.phase, Phase::Break);
        assert_eq!(app.timer.remaining.as_secs(), 300);
        assert_eq!((records, announcements), (1, 1));
        assert_eq!(app.summary_text(), "Completed 1 focus session · 26 min");
    }

    #[test]
    fn start_records_and_advances_while_quit_records_and_stops() {
        for mode in [OnEnd::Start, OnEnd::Quit] {
            let mut app = test_app();
            app.timer.config.on_end = mode;
            app.timer.remaining = std::time::Duration::ZERO;
            assert!(app.tick_with(|_| Ok((1, 25)), |_| {}));
            assert_eq!(app.should_quit, mode == OnEnd::Quit);
            assert_eq!(
                app.timer.phase,
                if mode == OnEnd::Start {
                    Phase::Break
                } else {
                    Phase::Work
                }
            );
            assert_eq!(app.completed_sessions, 1);
            if mode == OnEnd::Quit {
                assert!(!app.tick_with(|_| panic!("duplicate record"), |_| {}));
            }
        }
    }

    #[test]
    fn skipped_phases_bypass_end_action_and_preserve_pause() {
        for mode in [OnEnd::Ask, OnEnd::Quit] {
            let mut app = test_app();
            app.timer.config.on_end = mode;
            app.on_key('p');
            app.on_key('s');
            app.tick_with(|_| panic!("skip must not record"), |_| {});
            assert_eq!(app.timer.phase, Phase::Break);
            assert!(app.timer.paused);
            assert!(!app.should_quit);
            assert!(!app.waiting_for_next);
            assert_eq!(app.completed_sessions, 0);
        }
    }

    #[test]
    fn break_completion_waits_without_recording_and_s_continues() {
        let mut app = test_app();
        app.timer.config.on_end = OnEnd::Ask;
        app.on_key('b');
        app.timer.remaining = std::time::Duration::ZERO;
        app.tick_with(|_| panic!("break must not record"), |_| {});
        assert!(app.waiting_for_next);
        app.on_key('s');
        assert_eq!(app.timer.phase, Phase::Work);
        assert_eq!(app.timer.current_session, 2);
        assert!(!app.waiting_for_next);
    }

    #[test]
    fn persistence_failure_keeps_completion_and_exposes_unsaved_summary() {
        let mut app = test_app();
        app.timer.config.on_end = OnEnd::Ask;
        app.timer.remaining = std::time::Duration::ZERO;
        app.tick_with(|_| Err(std::io::Error::other("disk full").into()), |_| {});
        assert_eq!(app.completed_sessions, 1);
        assert!(app.status_message.as_ref().unwrap().contains("disk full"));
        assert!(app.summary_text().contains("1 not saved to statistics"));
        assert_eq!(app.cached_stats, (0, 0));
    }

    #[test]
    fn shared_key_events_support_quit_extension_and_ignore_releases() {
        for key in [
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        ] {
            let mut app = test_app();
            app.waiting_for_next = true;
            app.on_key_event(key);
            assert!(app.should_quit);
        }
        let mut app = test_app();
        app.on_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(app.timer.remaining.as_secs(), 1560);
        let mut key = KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE);
        key.kind = KeyEventKind::Release;
        app.on_key_event(key);
        assert!(!app.timer.paused);
    }
}

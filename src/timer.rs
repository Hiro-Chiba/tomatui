use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Work,
    Break,
    LongBreak,
}

impl Phase {
    pub fn label(&self) -> &str {
        match self {
            Phase::Work => "Work",
            Phase::Break => "Break",
            Phase::LongBreak => "Long Break",
        }
    }
}

pub struct TimerConfig {
    pub work_secs: u64,
    pub break_secs: u64,
    pub long_break_secs: u64,
    pub sessions: u32,
}

pub struct Timer {
    pub config: TimerConfig,
    pub phase: Phase,
    pub current_session: u32,
    pub remaining: Duration,
    pub total: Duration,
    pub paused: bool,
    pub skipped: bool,
    last_tick: Instant,
}

impl Timer {
    pub fn new(config: TimerConfig) -> Self {
        let duration = Duration::from_secs(config.work_secs);
        Self {
            config,
            phase: Phase::Work,
            current_session: 1,
            remaining: duration,
            total: duration,
            paused: false,
            skipped: false,
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        self.tick_at(Instant::now())
    }

    fn tick_at(&mut self, now: Instant) -> bool {
        if self.skipped || self.remaining.is_zero() {
            self.remaining = Duration::ZERO;
            return true;
        }

        if self.paused {
            self.last_tick = now;
            return false;
        }

        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;

        if elapsed >= self.remaining {
            self.remaining = Duration::ZERO;
            return true; // phase complete
        }

        self.remaining = self.remaining.saturating_sub(elapsed);
        false
    }

    pub fn advance_phase(&mut self) {
        match self.phase {
            Phase::Work => {
                if self.current_session >= self.config.sessions {
                    self.phase = Phase::LongBreak;
                    self.remaining = Duration::from_secs(self.config.long_break_secs);
                    self.total = self.remaining;
                } else {
                    self.phase = Phase::Break;
                    self.remaining = Duration::from_secs(self.config.break_secs);
                    self.total = self.remaining;
                }
            }
            Phase::Break => {
                self.current_session += 1;
                self.phase = Phase::Work;
                self.remaining = Duration::from_secs(self.config.work_secs);
                self.total = self.remaining;
            }
            Phase::LongBreak => {
                self.current_session = 1;
                self.phase = Phase::Work;
                self.remaining = Duration::from_secs(self.config.work_secs);
                self.total = self.remaining;
            }
        }
        self.skipped = false;
        self.last_tick = Instant::now();
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.last_tick = Instant::now();
    }

    pub fn skip(&mut self) {
        self.skipped = true;
        self.remaining = Duration::ZERO;
    }

    pub fn switch_to_work(&mut self) {
        if self.phase != Phase::Work {
            self.advance_phase();
            return;
        }

        self.phase = Phase::Work;
        self.remaining = Duration::from_secs(self.config.work_secs);
        self.total = self.remaining;
        self.skipped = false;
        self.last_tick = Instant::now();
    }

    pub fn switch_to_break(&mut self) {
        if self.current_session >= self.config.sessions {
            self.phase = Phase::LongBreak;
            self.remaining = Duration::from_secs(self.config.long_break_secs);
        } else {
            self.phase = Phase::Break;
            self.remaining = Duration::from_secs(self.config.break_secs);
        }
        self.total = self.remaining;
        self.skipped = false;
        self.last_tick = Instant::now();
    }

    pub fn progress(&self) -> f64 {
        if self.total.is_zero() {
            return 1.0;
        }
        1.0 - (self.remaining.as_secs_f64() / self.total.as_secs_f64())
    }

    pub fn remaining_display(&self) -> String {
        let secs = self.display_seconds();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    pub(crate) fn display_seconds(&self) -> u64 {
        let partial_second = if self.remaining.subsec_nanos() > 0 {
            1
        } else {
            0
        };
        self.remaining.as_secs().saturating_add(partial_second)
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
    fn test_advance_work_to_break() {
        let mut timer = Timer::new(test_config());
        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 1);

        timer.advance_phase();
        assert_eq!(timer.phase, Phase::Break);
        assert_eq!(timer.remaining, Duration::from_secs(300));
    }

    #[test]
    fn test_advance_break_to_work_increments_session() {
        let mut timer = Timer::new(test_config());
        timer.advance_phase(); // Work -> Break
        timer.advance_phase(); // Break -> Work (session 2)
        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 2);
    }

    #[test]
    fn test_advance_last_session_to_long_break() {
        let mut timer = Timer::new(test_config());
        // Advance to session 4
        for _ in 0..3 {
            timer.advance_phase(); // Work -> Break
            timer.advance_phase(); // Break -> Work
        }
        assert_eq!(timer.current_session, 4);
        assert_eq!(timer.phase, Phase::Work);

        timer.advance_phase(); // Work -> LongBreak (session 4)
        assert_eq!(timer.phase, Phase::LongBreak);
        assert_eq!(timer.remaining, Duration::from_secs(900));
    }

    #[test]
    fn test_advance_long_break_resets_session() {
        let mut timer = Timer::new(test_config());
        for _ in 0..3 {
            timer.advance_phase();
            timer.advance_phase();
        }
        timer.advance_phase(); // -> LongBreak
        timer.advance_phase(); // -> Work (session 1)
        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 1);
    }

    #[test]
    fn test_single_session_uses_long_break_and_resets_session() {
        let mut config = test_config();
        config.sessions = 1;
        let mut timer = Timer::new(config);

        timer.advance_phase();
        assert_eq!(timer.phase, Phase::LongBreak);
        assert_eq!(timer.current_session, 1);

        timer.advance_phase();
        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 1);
    }

    #[test]
    fn test_pause_excludes_paused_time_after_resume() {
        let mut timer = Timer::new(test_config());
        timer.toggle_pause();
        let paused_at = timer.last_tick;

        assert!(!timer.tick_at(paused_at + Duration::from_secs(10)));
        assert_eq!(timer.remaining, Duration::from_secs(1500));

        timer.toggle_pause();
        let resumed_at = timer.last_tick;
        assert!(!timer.tick_at(resumed_at + Duration::from_secs(1)));
        assert_eq!(timer.remaining, Duration::from_secs(1499));
    }

    #[test]
    fn test_tick_completes_at_the_deadline() {
        let mut timer = Timer::new(test_config());
        timer.remaining = Duration::from_secs(2);
        let started_at = timer.last_tick;

        assert!(timer.tick_at(started_at + Duration::from_secs(2)));
        assert!(timer.remaining.is_zero());
    }

    #[test]
    fn test_skip_completes_while_paused_and_preserves_pause() {
        let mut timer = Timer::new(test_config());
        timer.toggle_pause();
        timer.skip();

        assert!(timer.tick());
        timer.advance_phase();

        assert_eq!(timer.phase, Phase::Break);
        assert!(timer.paused);
        assert!(!timer.skipped);
    }

    #[test]
    fn test_zero_remaining_completes_while_paused_without_skip() {
        let mut timer = Timer::new(test_config());
        timer.toggle_pause();
        timer.remaining = Duration::ZERO;

        assert!(timer.tick());
        assert!(!timer.skipped);
    }

    #[test]
    fn test_skip_break_advances_session() {
        let mut timer = Timer::new(test_config());
        timer.advance_phase();
        timer.skip();

        assert!(timer.tick());
        timer.advance_phase();

        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 2);
    }

    #[test]
    fn test_skip_long_break_resets_session() {
        let mut timer = Timer::new(test_config());
        timer.current_session = timer.config.sessions;
        timer.advance_phase();
        timer.skip();

        assert!(timer.tick());
        timer.advance_phase();

        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 1);
    }

    #[test]
    fn test_progress_at_start_is_zero() {
        let timer = Timer::new(test_config());
        assert!((timer.progress() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_progress_with_half_remaining() {
        let mut timer = Timer::new(test_config());
        timer.remaining = Duration::from_secs(750); // half of 1500
        assert!((timer.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_progress_when_total_is_zero() {
        let mut timer = Timer::new(test_config());
        timer.total = Duration::ZERO;
        assert!((timer.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_remaining_display_format() {
        let mut timer = Timer::new(test_config());
        timer.remaining = Duration::from_secs(1500);
        assert_eq!(timer.remaining_display(), "25:00");

        timer.remaining = Duration::from_secs(65);
        assert_eq!(timer.remaining_display(), "01:05");

        timer.remaining = Duration::from_secs(0);
        assert_eq!(timer.remaining_display(), "00:00");

        timer.remaining = Duration::from_millis(1);
        assert_eq!(timer.remaining_display(), "00:01");

        timer.remaining = Duration::from_millis(1001);
        assert_eq!(timer.remaining_display(), "00:02");
    }

    #[test]
    fn test_skip_sets_skipped_flag() {
        let mut timer = Timer::new(test_config());
        assert!(!timer.skipped);
        timer.skip();
        assert!(timer.skipped);
        assert!(timer.remaining.is_zero());
    }

    #[test]
    fn test_advance_phase_resets_skipped() {
        let mut timer = Timer::new(test_config());
        timer.skip();
        assert!(timer.skipped);
        timer.advance_phase();
        assert!(!timer.skipped);
    }

    #[test]
    fn test_switch_to_work_resets_skipped() {
        let mut timer = Timer::new(test_config());
        timer.skip();
        assert!(timer.skipped);
        timer.switch_to_work();
        assert!(!timer.skipped);
    }

    #[test]
    fn test_switch_to_work_from_break_increments_session() {
        let mut timer = Timer::new(test_config());
        timer.advance_phase();

        timer.switch_to_work();

        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 2);
        assert_eq!(timer.remaining, Duration::from_secs(1500));
    }

    #[test]
    fn test_switch_to_work_from_long_break_resets_session() {
        let mut timer = Timer::new(test_config());
        timer.current_session = timer.config.sessions;
        timer.advance_phase();

        timer.switch_to_work();

        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 1);
        assert_eq!(timer.remaining, Duration::from_secs(1500));
    }

    #[test]
    fn test_switch_to_work_from_work_keeps_session() {
        let mut timer = Timer::new(test_config());
        timer.current_session = 3;
        timer.remaining = Duration::from_secs(1);

        timer.switch_to_work();

        assert_eq!(timer.phase, Phase::Work);
        assert_eq!(timer.current_session, 3);
        assert_eq!(timer.remaining, Duration::from_secs(1500));
    }

    #[test]
    fn test_switch_to_break_resets_skipped() {
        let mut timer = Timer::new(test_config());
        timer.skip();
        assert!(timer.skipped);
        timer.switch_to_break();
        assert!(!timer.skipped);
    }

    #[test]
    fn test_switch_to_break_at_last_session_uses_long_break() {
        let mut timer = Timer::new(test_config());
        timer.current_session = timer.config.sessions;

        timer.switch_to_break();

        assert_eq!(timer.phase, Phase::LongBreak);
        assert_eq!(timer.remaining, Duration::from_secs(900));
    }
}

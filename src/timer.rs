use crate::constants::SECS_PER_MIN;
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
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.paused {
            self.last_tick = Instant::now();
            return false;
        }

        let now = Instant::now();
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
        self.last_tick = Instant::now();
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.last_tick = Instant::now();
    }

    pub fn skip(&mut self) {
        self.remaining = Duration::ZERO;
    }

    pub fn switch_to_work(&mut self) {
        self.phase = Phase::Work;
        self.remaining = Duration::from_secs(self.config.work_secs);
        self.total = self.remaining;
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
        self.last_tick = Instant::now();
    }

    pub fn progress(&self) -> f64 {
        if self.total.is_zero() {
            return 1.0;
        }
        1.0 - (self.remaining.as_secs_f64() / self.total.as_secs_f64())
    }

    pub fn remaining_display(&self) -> String {
        let secs = self.remaining.as_secs();
        format!("{:02}:{:02}", secs / SECS_PER_MIN, secs % SECS_PER_MIN)
    }
}

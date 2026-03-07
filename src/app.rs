use crate::constants::{DATE_FORMAT, SECS_PER_MIN};
use crate::notification::bell;
use crate::stats::record_pomodoro;
use crate::timer::{Phase, Timer, TimerConfig};

pub struct App {
    pub timer: Timer,
    pub should_quit: bool,
}

impl App {
    pub fn new(config: TimerConfig) -> Self {
        Self {
            timer: Timer::new(config),
            should_quit: false,
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

        if self.timer.phase == Phase::Work {
            let work_minutes = self.timer.config.work_secs / SECS_PER_MIN;
            record_pomodoro(work_minutes);
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

    pub fn today_stats(&self) -> (u32, u64) {
        let stats = crate::stats::load_stats();
        let today = chrono::Local::now().format(DATE_FORMAT).to_string();
        stats.get_day(&today)
    }
}

use std::time::Duration;

pub const APP_NAME: &str = "tomatui";
pub const APP_DISPLAY_NAME: &str = "Tomatui";
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TICK_RATE: Duration = Duration::from_millis(100);
pub const SECONDS_PER_MINUTE: u64 = 60;
pub const MINUTES_PER_HOUR: u64 = 60;
pub const FULL_BLOCK: &str = "\u{2588}";
pub const PAUSED_LABEL: &str = " PAUSED";
pub const QUIT_KEY: char = 'q';
pub const PAUSE_KEY: char = 'p';
pub const SPACE_KEY: char = ' ';
pub const SKIP_KEY: char = 's';
pub const WORK_KEY: char = 'w';
pub const BREAK_KEY: char = 'b';

pub const BOX_WIDTH: u16 = 52;
pub const BOX_HEIGHT: u16 = 18;
pub const FONT_GLYPH_WIDTH: u16 = 8;
pub const FONT_VISUAL_OFFSET: u16 = 1;
pub const MINI_BAR_WIDTH: usize = 16;

use std::time::Duration;

// App identity
pub const APP_NAME: &str = "tomatui";

// Date format used for stats keys and display
pub const DATE_FORMAT: &str = "%Y-%m-%d";

// Event loop poll interval
pub const TICK_RATE: Duration = Duration::from_millis(100);

// TUI layout
pub const BOX_WIDTH: u16 = 52;
pub const BOX_HEIGHT: u16 = 18;
pub const FONT_GLYPH_WIDTH: u16 = 8;
pub const FONT_VISUAL_OFFSET: u16 = 1;

// Minimal mode
pub const MINI_BAR_WIDTH: usize = 16;

// Time conversion
pub const SECS_PER_MIN: u64 = 60;
pub const MINS_PER_HOUR: u64 = 60;

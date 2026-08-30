use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};
use std::io::{self, Write};

use crate::app::App;
use crate::constants::{FULL_BLOCK, MINI_BAR_WIDTH, PAUSED_LABEL, TICK_RATE};
use crate::timer::{Phase, TimerConfig};

const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_CLEAR_TO_END: &str = "\x1b[K";
const LIGHT_SHADE: &str = "\u{2591}";

fn phase_color_code(phase: Phase) -> &'static str {
    match phase {
        Phase::Work => ANSI_RED,
        Phase::Break => ANSI_GREEN,
        Phase::LongBreak => ANSI_BLUE,
    }
}

fn render_line(app: &App) -> io::Result<()> {
    let color = phase_color_code(app.timer.phase);
    let progress = app.timer.progress();
    let filled = (progress * MINI_BAR_WIDTH as f64) as usize;
    let empty = MINI_BAR_WIDTH - filled;
    let bar = format!(
        "{}{}{}",
        FULL_BLOCK.repeat(filled),
        LIGHT_SHADE.repeat(empty),
        ANSI_RESET
    );
    let pause = if app.timer.paused { PAUSED_LABEL } else { "" };

    let line = format!(
        "\r{}{} [{}] {}{} ({}/{}){}{}",
        color,
        app.timer.phase.label(),
        app.timer.remaining_display(),
        bar,
        color,
        app.timer.current_session,
        app.timer.config.sessions,
        pause,
        ANSI_CLEAR_TO_END,
    );

    let mut stdout = io::stdout();
    write!(stdout, "{}{}", line, ANSI_RESET)?;
    stdout.flush()
}

fn restore_terminal() -> io::Result<()> {
    let show_cursor = execute!(io::stdout(), cursor::Show);
    let clear_line = execute!(io::stdout(), terminal::Clear(ClearType::CurrentLine));
    let disable_raw_mode = terminal::disable_raw_mode();
    let print_newline = writeln!(io::stdout());

    show_cursor?;
    clear_line?;
    disable_raw_mode?;
    print_newline
}

pub fn run(config: TimerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(config);
    if let Err(error) = terminal::enable_raw_mode() {
        let _ = terminal::disable_raw_mode();
        return Err(error.into());
    }

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        previous_hook(panic_info);
    }));

    if let Err(error) = execute!(io::stdout(), cursor::Hide) {
        let _ = execute!(io::stdout(), cursor::Show);
        let _ = terminal::disable_raw_mode();
        return Err(error.into());
    }

    let result = (|| -> io::Result<()> {
        let mut redraw = true;

        loop {
            if redraw {
                render_line(&app)?;
                redraw = false;
            }

            if event::poll(TICK_RATE)? {
                redraw = true;
                if let Event::Key(key) = event::read()?
                    && key.kind == KeyEventKind::Press
                {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Esc => app.should_quit = true,
                        _ => {}
                    }
                }
            }

            if app.should_quit {
                break;
            }

            if app.tick() {
                redraw = true;
            }
        }

        Ok(())
    })();

    let restore_result = restore_terminal();
    result?;
    restore_result?;
    Ok(())
}

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};
use std::io::{self, Write};

use crate::app::App;
use crate::constants::{MINI_BAR_WIDTH, TICK_RATE};
use crate::timer::{Phase, TimerConfig};

fn phase_color_code(phase: Phase) -> &'static str {
    match phase {
        Phase::Work => "\x1b[31m",    // red
        Phase::Break => "\x1b[32m",   // green
        Phase::LongBreak => "\x1b[34m", // blue
    }
}

fn render_line(app: &App) {
    let reset = "\x1b[0m";
    let color = phase_color_code(app.timer.phase);
    let progress = app.timer.progress();
    let filled = (progress * MINI_BAR_WIDTH as f64) as usize;
    let empty = MINI_BAR_WIDTH - filled;
    let bar = format!(
        "{}{}{}",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty),
        reset
    );
    let pause = if app.timer.paused { " PAUSED" } else { "" };

    let line = format!(
        "\r{}{} [{}] {}{} ({}/{}){}\x1b[K",
        color,
        app.timer.phase.label(),
        app.timer.remaining_display(),
        bar,
        color,
        app.timer.current_session,
        app.timer.config.sessions,
        pause,
    );

    print!("{}{}", line, reset);
    io::stdout().flush().ok();
}

pub fn run(config: TimerConfig) -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide)?;

    let mut app = App::new(config);

    loop {
        render_line(&app);

        if event::poll(TICK_RATE)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Esc => app.should_quit = true,
                    _ => {}
                }
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    // Clean up
    execute!(
        io::stdout(),
        cursor::Show,
        terminal::Clear(ClearType::CurrentLine)
    )?;
    terminal::disable_raw_mode()?;
    println!();
    Ok(())
}

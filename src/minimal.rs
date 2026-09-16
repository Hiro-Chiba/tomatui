use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};
use std::io::{self, Write};

use crate::app::App;
use crate::constants::{FULL_BLOCK, MINI_BAR_WIDTH, TICK_RATE};
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

// Leave the last column unused to avoid terminal autowrap.
fn line_for_width(app: &App, width: u16) -> String {
    let limit = usize::from(width.saturating_sub(1));
    let time = app.timer.remaining_display();
    let state = if app.waiting_for_next {
        "complete"
    } else if app.timer.paused {
        "paused"
    } else {
        ""
    };
    let mut line = format!("{time} {} {state}", app.timer.phase.label())
        .trim_end()
        .to_owned();
    if let Some(message) = &app.status_message {
        line.push_str(&format!(" | {message}"));
    } else if app.waiting_for_next {
        line.push_str(" | Enter/s next  q quit");
    } else {
        let filled = (app.timer.progress().clamp(0.0, 1.0) * MINI_BAR_WIDTH as f64) as usize;
        let bar = format!(
            "{}{}",
            FULL_BLOCK.repeat(filled),
            LIGHT_SHADE.repeat(MINI_BAR_WIDTH - filled)
        );
        let details = format!(
            "  {bar}  {}/{}  p pause  + 1m  q quit",
            app.timer.current_session, app.timer.config.sessions
        );
        if ratatui::text::Span::raw(&line).width() + ratatui::text::Span::raw(&details).width()
            <= limit
        {
            line.push_str(&details);
        } else {
            line.push_str("  p pause  q quit");
        }
    }
    let mut used = 0;
    line.chars()
        .filter(|character| !character.is_control())
        .take_while(|character| {
            used += ratatui::text::Span::raw(character.to_string()).width();
            used <= limit
        })
        .collect()
}

fn render_line(app: &App) -> io::Result<()> {
    let color = phase_color_code(app.timer.phase);
    let (width, _) = terminal::size()?;
    let mut stdout = io::stdout();
    write!(
        stdout,
        "\r{color}{}{ANSI_RESET}{ANSI_CLEAR_TO_END}",
        line_for_width(app, width)
    )?;
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
                    app.on_key_event(key);
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
    println!("{}", app.summary_text());
    if let Some(message) = &app.status_message {
        eprintln!("{message}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OnEnd;

    fn app() -> App {
        App::new(TimerConfig {
            work_secs: 1500,
            break_secs: 300,
            long_break_secs: 900,
            sessions: 4,
            on_end: OnEnd::Start,
        })
    }

    #[test]
    fn line_never_wraps_even_in_a_one_column_pane() {
        let mut app = app();
        for waiting in [false, true] {
            app.waiting_for_next = waiting;
            for width in 0..120 {
                let line = line_for_width(&app, width);
                assert!(
                    ratatui::text::Span::raw(line).width() <= usize::from(width.saturating_sub(1))
                );
            }
        }
        assert!(line_for_width(&app, 80).contains("Enter/s next"));
    }

    #[test]
    fn errors_with_wide_characters_fit_without_control_sequences() {
        let mut app = app();
        app.status_message = Some("保存失敗\nTry again".into());
        let line = line_for_width(&app, 28);
        assert!(ratatui::text::Span::raw(&line).width() <= 27);
        assert!(!line.contains('\n'));
    }
}

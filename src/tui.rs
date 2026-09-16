use crossterm::event::{self, Event};

use crate::app::App;
use crate::timer::TimerConfig;
use crate::ui;

pub fn run(config: TimerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(config);
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(error) => {
            ratatui::restore();
            return Err(error.into());
        }
    };
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut redraw = true;

        loop {
            if redraw {
                terminal.draw(|frame| ui::draw(frame, &mut app))?;
                redraw = false;
            }

            let input = if event::poll(app.poll_timeout())? {
                Some(event::read()?)
            } else {
                None
            };

            // Account for elapsed time before pause or phase-switch input resets the clock.
            redraw |= app.tick();
            if app.should_quit {
                break;
            }
            match input {
                Some(Event::Key(key)) => redraw |= app.on_key_event(key),
                Some(Event::Resize(_, _)) => redraw = true,
                _ => {}
            }
            // Apply skips immediately, including while paused.
            redraw |= app.tick();
            if app.should_quit {
                break;
            }
        }

        Ok(())
    })();

    let restore_result = ratatui::try_restore();
    result?;
    restore_result?;
    println!("{}", app.summary_text());
    if let Some(message) = &app.status_message {
        eprintln!("{message}");
    }
    Ok(())
}

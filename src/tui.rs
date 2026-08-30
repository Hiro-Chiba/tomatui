use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::App;
use crate::constants::TICK_RATE;
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

            if event::poll(TICK_RATE)? {
                redraw = true;
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char(c) => app.on_key(c),
                            KeyCode::Esc => app.should_quit = true,
                            _ => {}
                        }
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

    ratatui::restore();
    result
}

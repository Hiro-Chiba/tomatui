use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::App;
use crate::constants::TICK_RATE;
use crate::timer::TimerConfig;
use crate::ui;

pub fn run(config: TimerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new(config);

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

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

    ratatui::restore();
    Ok(())
}

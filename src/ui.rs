use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use tui_big_text::{BigText, PixelSize};

use crate::app::App;
use crate::constants::{BOX_HEIGHT, BOX_WIDTH, FONT_GLYPH_WIDTH, FULL_BLOCK, MINUTES_PER_HOUR};
use crate::timer::Phase;

fn phase_color(phase: Phase) -> Color {
    match phase {
        Phase::Work => Color::Rgb(235, 87, 87),
        Phase::Break => Color::Rgb(111, 207, 151),
        Phase::LongBreak => Color::Rgb(86, 156, 214),
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [row] = Layout::vertical([Constraint::Length(height.min(area.height))])
        .flex(Flex::Center)
        .areas(area);
    Layout::horizontal([Constraint::Length(width.min(area.width))])
        .flex(Flex::Center)
        .areas::<1>(row)[0]
}

fn text(frame: &mut Frame, area: Rect, content: impl Into<Line<'static>>, color: Color) {
    let content = content.into();
    let area = centered(area, content.width().min(area.width as usize) as u16, 1);
    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Left)
            .style(Style::default().fg(color)),
        area,
    );
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered(frame.area(), BOX_WIDTH, BOX_HEIGHT);
    let color = phase_color(app.timer.phase);
    let time = app.timer.remaining_display();
    let state = if app.waiting_for_next {
        "complete"
    } else if app.timer.paused {
        "paused"
    } else {
        ""
    };
    let phase = format!("{}  {}", app.timer.phase.label(), state)
        .trim_end()
        .to_owned();
    let big_width = time.len() as u16 * FONT_GLYPH_WIDTH;

    // Preserve the countdown and controls when a terminal pane is too small for the large clock.
    if area.width < big_width || area.height < BOX_HEIGHT {
        let rows: [Rect; 4] = std::array::from_fn(|index| {
            let offset = (index as u16).min(area.height);
            Rect::new(
                area.x,
                area.y + offset,
                area.width,
                u16::from(offset < area.height),
            )
        });
        frame.render_widget(
            Paragraph::new(format!("{time}  {phase}")).style(Style::default().fg(color)),
            rows[0],
        );
        text(
            frame,
            rows[1],
            format!(
                "Session {} / {}",
                app.timer.current_session, app.timer.config.sessions
            ),
            Color::Gray,
        );
        text(
            frame,
            rows[2],
            if app.waiting_for_next {
                "Enter next  s next  q quit"
            } else {
                "p pause  + 1m  s skip  q quit"
            },
            Color::Gray,
        );
        if let Some(message) = &app.status_message {
            text(frame, rows[3], message.clone(), Color::Yellow);
        }
        return;
    }

    let rows = Layout::vertical([
        Constraint::Length(1), // phase
        Constraint::Length(1),
        Constraint::Length(4), // clock
        Constraint::Length(1),
        Constraint::Length(1), // progress
        Constraint::Length(1),
        Constraint::Length(1), // sessions
        Constraint::Length(1), // today
        Constraint::Length(1),
        Constraint::Length(1), // controls
        Constraint::Length(1), // extra controls
        Constraint::Length(1), // error
    ])
    .split(area);

    text(frame, rows[0], phase, color);
    // Bitmap glyphs include blank columns. Center the visible clock, not its padding.
    let clock_area = Rect::new(0, 0, big_width, 4);
    let mut clock = Buffer::empty(clock_area);
    BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .style(Style::new().fg(color).bold())
        .lines(vec![time.into()])
        .build()
        .render(clock_area, &mut clock);
    let occupied = |x| (0..4).any(|y| clock[(x, y)].symbol() != " ");
    if let (Some(left), Some(right)) = (
        (0..big_width).find(|&x| occupied(x)),
        (0..big_width).rfind(|&x| occupied(x)),
    ) {
        let target = centered(rows[2], right - left + 1, 4);
        for y in 0..target.height {
            for x in 0..target.width {
                frame.buffer_mut()[(target.x + x, target.y + y)] = clock[(left + x, y)].clone();
            }
        }
    }
    let bar_width = 36;
    let filled = (app.timer.progress().clamp(0.0, 1.0) * bar_width as f64) as usize;
    text(
        frame,
        rows[4],
        format!(
            "{}{}",
            FULL_BLOCK.repeat(filled),
            "░".repeat(bar_width - filled)
        ),
        color,
    );
    text(
        frame,
        rows[6],
        format!(
            "Session {} / {}",
            app.timer.current_session, app.timer.config.sessions
        ),
        Color::Gray,
    );
    let (pomos, minutes) = app.today_stats();
    text(
        frame,
        rows[7],
        format!(
            "Today  {pomos} pomodoros  ·  {}h {}m",
            minutes / MINUTES_PER_HOUR,
            minutes % MINUTES_PER_HOUR
        ),
        Color::Rgb(126, 139, 157),
    );
    let help = if app.waiting_for_next {
        vec![("enter", " next   "), ("s", " next   "), ("q", " quit")]
    } else {
        vec![
            ("p/space", " pause   "),
            ("+", " add 1m   "),
            ("s", " skip"),
        ]
    };
    let spans = help
        .into_iter()
        .flat_map(|(key, label)| {
            [
                Span::styled(key, Style::default().fg(color)),
                Span::styled(label, Style::default().fg(Color::Gray)),
            ]
        })
        .collect::<Vec<_>>();
    text(frame, rows[9], Line::from(spans), Color::Gray);
    if !app.waiting_for_next {
        text(frame, rows[10], "w/b switch   q quit", Color::DarkGray);
    }
    if let Some(message) = &app.status_message {
        text(frame, rows[11], message.clone(), Color::Yellow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::OnEnd, timer::TimerConfig};
    use ratatui::{Terminal, backend::TestBackend};

    fn app() -> App {
        App::new(TimerConfig {
            work_secs: 1500,
            break_secs: 300,
            long_break_secs: 900,
            sessions: 4,
            on_end: OnEnd::Start,
        })
    }

    fn render(width: u16, height: u16, app: &mut App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| draw(frame, app)).unwrap();
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn visible_clock_and_labels_share_the_center() {
        for width in [60, 61, 80, 81] {
            for seconds in [1500, 899, 671, 0] {
                let mut app = app();
                app.timer.remaining = std::time::Duration::from_secs(seconds);
                let mut terminal = Terminal::new(TestBackend::new(width, 24)).unwrap();
                terminal.draw(|frame| draw(frame, &mut app)).unwrap();
                let buffer = terminal.backend().buffer();
                let area = centered(buffer.area, BOX_WIDTH, BOX_HEIGHT);
                // Treat all four clock rows as one visible shape.
                for (offset, height) in [(0, 1), (2, 4), (7, 1), (9, 1), (10, 1), (12, 1), (13, 1)]
                {
                    let visible = |x| {
                        (area.y + offset..area.y + offset + height)
                            .any(|y| buffer[(x, y)].symbol() != " ")
                    };
                    let left = (area.x..area.right()).find(|&x| visible(x)).unwrap();
                    let right = (area.x..area.right()).rfind(|&x| visible(x)).unwrap();
                    let left_margin = left - area.x;
                    let right_margin = area.right() - right - 1;
                    assert!(
                        left_margin.abs_diff(right_margin) <= 1,
                        "width={width}, seconds={seconds}, row={offset}: {left_margin} / {right_margin}"
                    );
                }
            }
        }
    }

    #[test]
    fn clock_survives_narrow_and_tiny_terminals() {
        let mut app = app();
        for (width, height) in [
            (1, 1),
            (0, 0),
            (5, 1),
            (20, 4),
            (39, 12),
            (40, 14),
            (40, 15),
            (40, 17),
            (80, 24),
        ] {
            let output = render(width, height, &mut app);
            assert_eq!(
                output.chars().count(),
                usize::from(width) * usize::from(height)
            );
            if width >= 5 && height < BOX_HEIGHT {
                assert!(output.contains("25:00"));
            }
        }
    }

    #[test]
    fn waiting_shows_next_controls_in_both_layouts() {
        let mut app = app();
        app.waiting_for_next = true;
        app.timer.remaining = std::time::Duration::ZERO;
        for (width, height) in [(36, 5), (80, 24)] {
            let output = render(width, height, &mut app);
            assert!(output.contains("complete"));
            assert!(output.contains("next"));
            assert!(!output.contains("pause"));
        }
    }

    #[test]
    fn persistence_errors_are_visible() {
        let mut app = app();
        app.status_message = Some("Could not save statistics".into());
        for (width, height) in [(36, 5), (80, 24)] {
            assert!(render(width, height, &mut app).contains("Could not save statistics"));
        }
    }
}

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Padding, Paragraph};
use tui_big_text::{BigText, PixelSize};

use crate::app::App;
use crate::constants::{BOX_HEIGHT, BOX_WIDTH, FONT_GLYPH_WIDTH, FONT_VISUAL_OFFSET};
use crate::timer::Phase;

fn phase_color(phase: Phase) -> Color {
    match phase {
        Phase::Work => Color::Rgb(235, 87, 87),
        Phase::Break => Color::Rgb(111, 207, 151),
        Phase::LongBreak => Color::Rgb(86, 156, 214),
    }
}

fn dim_color(phase: Phase) -> Color {
    match phase {
        Phase::Work => Color::Rgb(100, 30, 30),
        Phase::Break => Color::Rgb(30, 80, 50),
        Phase::LongBreak => Color::Rgb(30, 50, 80),
    }
}

fn center_area(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    let horizontal = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0]);
    horizontal[0]
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let color = phase_color(app.timer.phase);
    let bg_dim = dim_color(app.timer.phase);

    let outer = center_area(frame.area(), BOX_WIDTH, BOX_HEIGHT);

    // Outer block with rounded borders
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .border_style(Style::default().fg(color))
        .padding(Padding::new(1, 1, 0, 0));
    let inner = outer_block.inner(outer);
    frame.render_widget(outer_block, outer);

    let [
        title_area,
        sep1,
        phase_area,
        time_area,
        sep2,
        gauge_area,
        sep3,
        session_dots_area,
        sep4,
        stats_area,
        sep5,
        help_area,
    ] = Layout::vertical([
        Constraint::Length(1), // title
        Constraint::Length(1), // sep
        Constraint::Length(1), // phase
        Constraint::Length(4), // big time (HalfHeight = 4 rows)
        Constraint::Length(1), // sep
        Constraint::Length(1), // gauge
        Constraint::Length(1), // sep
        Constraint::Length(1), // session dots
        Constraint::Length(1), // sep
        Constraint::Length(1), // stats
        Constraint::Length(1), // sep
        Constraint::Length(1), // help
    ])
    .areas(inner);

    // Title
    let title = Paragraph::new(Line::from(Span::styled(
        " POMODORO ",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(title, title_area);

    // Separator helper
    let sep_line = "\u{2500}".repeat(inner.width as usize);
    let sep_style = Style::default().fg(bg_dim);
    let render_sep = |frame: &mut Frame, area: Rect, line: &str, style: Style| {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(line.to_string(), style))),
            area,
        );
    };
    render_sep(frame, sep1, &sep_line, sep_style);

    // Phase
    let pause_indicator = if app.timer.paused {
        Span::styled(
            " PAUSED",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::SLOW_BLINK),
        )
    } else {
        Span::raw("")
    };
    let phase = Paragraph::new(Line::from(vec![
        Span::styled(
            app.timer.phase.label().to_uppercase(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        pause_indicator,
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(phase, phase_area);

    // Big time display using tui-big-text
    // HalfHeight: each char = 8 cols wide, "00:00" = 5 chars = 40 cols
    let time_str = app.timer.remaining_display();
    let big_width = (time_str.len() as u16) * FONT_GLYPH_WIDTH;
    // font8x8 glyphs are left-aligned within cells (~3px empty on right)
    // offset compensates for the visual weight shift
    let time_x = time_area.x + (time_area.width.saturating_sub(big_width)) / 2 + FONT_VISUAL_OFFSET;
    let time_centered = Rect::new(time_x, time_area.y, big_width, time_area.height);
    let big_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .style(Style::new().fg(color).bold())
        .lines(vec![time_str.into()])
        .build();
    frame.render_widget(big_text, time_centered);

    render_sep(frame, sep2, &sep_line, sep_style);

    // Progress gauge
    let progress = app.timer.progress();
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(bg_dim))
        .ratio(progress.clamp(0.0, 1.0))
        .label(format!("{:.0}%", progress * 100.0));
    frame.render_widget(gauge, gauge_area);

    render_sep(frame, sep3, &sep_line, sep_style);

    // Session dots
    let total = app.timer.config.sessions;
    let current = app.timer.current_session;
    let session_widget = if total <= u32::from(session_dots_area.width / 2) {
        let dots: Vec<Span> = (1..=total)
            .map(|i| {
                if i < current || (i == current && app.timer.phase != Phase::Work) {
                    Span::styled("\u{25cf} ", Style::default().fg(color))
                } else if i == current {
                    Span::styled(
                        "\u{25ce} ",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled("\u{25cb} ", Style::default().fg(Color::DarkGray))
                }
            })
            .collect();
        Paragraph::new(Line::from(dots))
    } else {
        Paragraph::new(format!("Session {current} / {total}"))
            .style(Style::default().fg(Color::White))
    }
    .alignment(Alignment::Center);
    frame.render_widget(session_widget, session_dots_area);

    render_sep(frame, sep4, &sep_line, sep_style);

    // Today's stats
    let (pomos, minutes) = app.today_stats();
    let hours = minutes / 60;
    let mins = minutes % 60;
    let stats_widget = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("{} pomodoros", pomos),
            Style::default().fg(Color::White),
        ),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}h {}m today", hours, mins),
            Style::default().fg(Color::White),
        ),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(stats_widget, stats_area);

    render_sep(frame, sep5, &sep_line, sep_style);

    // Help
    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            "q",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" quit  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "p",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("/", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "space",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" pause  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "s",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" skip  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "w",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("/", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "b",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" switch", Style::default().fg(Color::DarkGray)),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(help, help_area);
}

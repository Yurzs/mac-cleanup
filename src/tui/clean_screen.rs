use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Padding, Paragraph};

use super::theme;
use super::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .title(" Cleaning ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    let (done, total) = app.clean_progress;
    let spinner = theme::SPINNER_FRAMES[app.spinner_frame % theme::SPINNER_FRAMES.len()];

    // Header.
    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(spinner, Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("  Cleaning up... ({done}/{total})"),
                Style::default().fg(Color::White),
            ),
        ]),
    ]);
    frame.render_widget(header, chunks[0]);

    // Progress bar.
    let ratio = if total > 0 {
        done as f64 / total as f64
    } else {
        0.0
    };
    let gauge = Gauge::default()
        .ratio(ratio)
        .gauge_style(Style::default().fg(Color::Green))
        .label(format!("{:.0}%", ratio * 100.0));
    frame.render_widget(gauge, chunks[2]);

    // Current item.
    if let Some(current) = &app.clean_current {
        let current_text = Paragraph::new(vec![Line::from(vec![Span::styled(
            format!("   {current}"),
            theme::DIM_STYLE,
        )])]);
        frame.render_widget(current_text, chunks[3]);
    }
}

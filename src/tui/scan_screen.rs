use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use super::theme;
use super::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .title(" mac-cleanup ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let spinner = theme::SPINNER_FRAMES[app.spinner_frame % theme::SPINNER_FRAMES.len()];
    let item_count = app.items.len();

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(spinner, Style::default().fg(Color::Cyan)),
            Span::raw("  Scanning your system for junk..."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("   Found {item_count} items so far"),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            format!("   {}", app.scan_progress),
            theme::DIM_STYLE,
        )]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::styled(
            "   Press q to quit",
            theme::HELP_STYLE,
        )]),
    ];

    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, inner);
}

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::util::size::format_size;

use super::theme;
use super::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .title(" Done ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Cleanup complete!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if let Some(stats) = &app.clean_stats {
        lines.push(Line::from(vec![
            Span::styled("  Freed:    ", Style::default().fg(Color::White)),
            Span::styled(
                format_size(stats.deleted_size),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Deleted:  ", Style::default().fg(Color::White)),
            Span::raw(format!("{} items", stats.deleted_count)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Time:     ", Style::default().fg(Color::White)),
            Span::raw(format!("{:.1}s", stats.duration.as_secs_f64())),
        ]));

        if stats.failed_count > 0 {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                format!("  {} items failed to delete", stats.failed_count),
                Style::default().fg(Color::Yellow),
            )]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Press any key to exit",
        theme::HELP_STYLE,
    )]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

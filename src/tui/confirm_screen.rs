use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::layout::{Layout, Direction, Constraint};

use crate::util::path::shorten_path;
use crate::util::size::format_size;

use super::theme;
use super::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Layout: main content + bottom help bar.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let block = Block::default()
        .title(" Confirm Cleanup ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(chunks[0]);
    frame.render_widget(block, chunks[0]);

    // Status bar.
    draw_help_bar(frame, chunks[1]);

    let selected = app.selected_items();
    let total_size = app.selected_size();

    let cmd_count = selected.iter().filter(|i| i.clean_command.is_some()).count();
    let delete_count = selected.len() - cmd_count;

    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!(
                "About to clean {} items ({})",
                selected.len(),
                format_size(total_size)
            ),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    // Warn about risky items.
    let caution_count = selected
        .iter()
        .filter(|i| i.risk == crate::rules::Risk::Caution)
        .count();
    let dangerous_count = selected
        .iter()
        .filter(|i| i.risk == crate::rules::Risk::Dangerous)
        .count();

    if dangerous_count > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("  WARNING: {dangerous_count} DANGEROUS items selected — may cause data loss!"),
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )]));
    }
    if caution_count > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("  Caution: {caution_count} items may contain data worth keeping"),
            Style::default().fg(Color::Yellow),
        )]));
    }

    // Summary of actions.
    if cmd_count > 0 {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{cmd_count} via native command"),
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                format!(",  {delete_count} via filesystem delete"),
                Style::default().fg(Color::Cyan),
            ),
        ]));
    }
    if caution_count > 0 || dangerous_count > 0 || cmd_count > 0 {
        lines.push(Line::from(""));
    }

    let visible_height = inner.height as usize;
    let header_lines = lines.len() + 1; // +1 for "... and N more" line
    let max_show = visible_height.saturating_sub(header_lines);

    for (i, item) in selected.iter().enumerate() {
        if i >= max_show {
            lines.push(Line::from(vec![Span::styled(
                format!("  ... and {} more", selected.len() - max_show),
                theme::DIM_STYLE,
            )]));
            break;
        }

        let action = format_action(item);

        let risk_color = match item.risk {
            crate::rules::Risk::Safe => theme::SAFE_COLOR,
            crate::rules::Risk::Caution => theme::CAUTION_COLOR,
            crate::rules::Risk::Dangerous => theme::DANGEROUS_COLOR,
        };

        lines.push(Line::from(vec![
            Span::styled("  - ", Style::default().fg(risk_color)),
            Span::styled(
                format!("{:>10}", format_size(item.size)),
                theme::SIZE_STYLE,
            ),
            Span::raw("  "),
            Span::styled(shorten_path(&item.path), theme::PATH_STYLE),
            Span::styled(format!("  {action}"), Style::default().fg(Color::DarkGray)),
        ]));
    }

    lines.push(Line::from(""));
    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn draw_help_bar(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let bar = Line::from(vec![
        Span::styled(
            " Enter: start cleanup  ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("|  Esc: go back", theme::HELP_STYLE),
    ]);
    frame.render_widget(Paragraph::new(bar), inner);
}

/// Format the action that will be taken for an item.
fn format_action(item: &crate::rules::JunkItem) -> String {
    if let Some(cmd) = &item.clean_command {
        format!("$ {}", cmd.join(" "))
    } else {
        "delete".into()
    }
}

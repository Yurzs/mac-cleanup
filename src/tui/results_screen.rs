use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::util::path::shorten_path;
use crate::util::size::format_size;

use super::App;
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Layout: main content + bottom help bar.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    draw_tree(frame, app, chunks[0]);
    draw_status_bar(frame, app, chunks[1]);
}

fn draw_tree(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Results ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .padding(Padding::new(1, 1, 0, 0));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_height = inner.height as usize;
    let total_rows = app.visible_row_count();

    // Auto-scroll to keep cursor visible.
    let scroll_offset = if app.cursor < app.scroll_offset {
        app.cursor
    } else if app.cursor >= app.scroll_offset + visible_height {
        app.cursor - visible_height + 1
    } else {
        app.scroll_offset
    };

    let mut lines: Vec<Line> = Vec::new();
    let mut row = 0;

    for cat in &app.categories {
        if row >= scroll_offset + visible_height {
            break;
        }

        // Category header line.
        if row >= scroll_offset {
            let is_cursor = row == app.cursor;
            let arrow = if cat.expanded {
                theme::CATEGORY_EXPANDED
            } else {
                theme::CATEGORY_COLLAPSED
            };
            let checkbox = if cat.selected {
                theme::CHECKBOX_ON
            } else {
                theme::CHECKBOX_OFF
            };

            let cat_size: u64 = cat.groups.iter().map(|g| g.total_size()).sum();
            let selected_size: u64 = cat.groups.iter().map(|g| g.selected_size()).sum();
            let count: usize = cat.groups.iter().map(|g| g.item_count()).sum();

            let style = if is_cursor {
                Style::default()
                    .fg(theme::category_color(cat.category))
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default()
                    .fg(theme::category_color(cat.category))
                    .add_modifier(Modifier::BOLD)
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{checkbox} {arrow}"), style),
                Span::styled(format!("{}", cat.category), style),
                Span::styled(
                    format!("  ({count} items, {})", format_size(cat_size)),
                    if is_cursor { style } else { theme::DIM_STYLE },
                ),
                if selected_size > 0 && selected_size != cat_size {
                    Span::styled(
                        format!("  [selected: {}]", format_size(selected_size)),
                        Style::default().fg(Color::Green),
                    )
                } else {
                    Span::raw("")
                },
            ]));
        }
        row += 1;

        // Groups (if category is expanded).
        if cat.expanded {
            for group in &cat.groups {
                if row >= scroll_offset + visible_height {
                    break;
                }

                if group.is_multi() {
                    // Multi-item group: render a collapsible group header.
                    if row >= scroll_offset {
                        let is_cursor = row == app.cursor;
                        let arrow = if group.expanded {
                            theme::GROUP_EXPANDED
                        } else {
                            theme::GROUP_COLLAPSED
                        };
                        let checkbox = if group.selected {
                            theme::CHECKBOX_ON
                        } else {
                            theme::CHECKBOX_OFF
                        };

                        let group_size = group.total_size();
                        let group_count = group.item_count();

                        // Use the risk of the first item for coloring the group.
                        let risk_style = match group.items[0].item.risk {
                            crate::rules::Risk::Safe => Style::default().fg(theme::SAFE_COLOR),
                            crate::rules::Risk::Caution => {
                                Style::default().fg(theme::CAUTION_COLOR)
                            }
                            crate::rules::Risk::Dangerous => {
                                Style::default().fg(theme::DANGEROUS_COLOR)
                            }
                        };

                        let line_style = if is_cursor {
                            Style::default().add_modifier(Modifier::REVERSED)
                        } else {
                            Style::default()
                        };

                        lines.push(Line::from(vec![
                            Span::styled(
                                format!("  {checkbox} {arrow}"),
                                if is_cursor { line_style } else { risk_style },
                            ),
                            Span::styled(
                                group.rule_name.to_string(),
                                if is_cursor {
                                    line_style
                                } else {
                                    Style::default().fg(Color::White)
                                },
                            ),
                            Span::styled(
                                format!(" ({group_count} items, {})", format_size(group_size)),
                                if is_cursor {
                                    line_style
                                } else {
                                    theme::DIM_STYLE
                                },
                            ),
                        ]));
                    }
                    row += 1;

                    // Individual items within expanded multi-item group.
                    if group.expanded {
                        for entry in &group.items {
                            if row >= scroll_offset + visible_height {
                                break;
                            }
                            if row >= scroll_offset {
                                let is_cursor = row == app.cursor;
                                render_item_line(&mut lines, entry, is_cursor, "    ");
                            }
                            row += 1;
                        }
                    }
                } else {
                    // Single-item group: render directly as the item (no group header).
                    let entry = &group.items[0];
                    if row >= scroll_offset {
                        let is_cursor = row == app.cursor;
                        render_item_line(&mut lines, entry, is_cursor, "  ");
                    }
                    row += 1;
                }
            }
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);

    // Scrollbar.
    if total_rows > visible_height {
        let mut scrollbar_state = ScrollbarState::new(total_rows).position(scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Render a single item line with the given indent prefix.
fn render_item_line<'a>(
    lines: &mut Vec<Line<'a>>,
    entry: &'a super::ItemEntry,
    is_cursor: bool,
    indent: &'static str,
) {
    let checkbox = if entry.selected {
        theme::CHECKBOX_ON
    } else {
        theme::CHECKBOX_OFF
    };

    let risk_style = match entry.item.risk {
        crate::rules::Risk::Safe => Style::default().fg(theme::SAFE_COLOR),
        crate::rules::Risk::Caution => Style::default().fg(theme::CAUTION_COLOR),
        crate::rules::Risk::Dangerous => Style::default().fg(theme::DANGEROUS_COLOR),
    };

    let line_style = if is_cursor {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    };

    let action_hint = if entry.item.clean_command.is_some() {
        " [cmd]"
    } else {
        ""
    };

    lines.push(Line::from(vec![
        Span::styled(
            format!("{indent}{checkbox} "),
            if is_cursor { line_style } else { risk_style },
        ),
        Span::styled(
            format!("{:<24}", entry.item.rule_name),
            if is_cursor {
                line_style
            } else {
                Style::default().fg(Color::White)
            },
        ),
        Span::styled(
            format!("{:>10}", format_size(entry.item.size)),
            if is_cursor {
                line_style
            } else {
                theme::SIZE_STYLE
            },
        ),
        Span::styled(
            format!("  {}", shorten_path(&entry.item.path)),
            if is_cursor {
                line_style
            } else {
                theme::PATH_STYLE
            },
        ),
        Span::styled(
            action_hint.to_string(),
            if is_cursor {
                line_style
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let selected = app.selected_count();
    let total_selected_size = app.selected_size();

    let scan_info = if let Some(stats) = &app.scan_stats {
        format!("Scanned in {:.1}s", stats.duration.as_secs_f64(),)
    } else {
        String::new()
    };

    let left = format!(
        " {selected} selected ({})  |  {scan_info}",
        format_size(total_selected_size),
    );

    let right = " Space:select  Tab:expand  a:all  n:none  Enter:clean  q:quit ";

    let bar = Line::from(vec![
        Span::styled(
            left,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(right, theme::HELP_STYLE),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(Paragraph::new(bar), inner);
}

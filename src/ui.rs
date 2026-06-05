use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let suggestion_height = if app.show_suggestions {
        app.filtered_suggestions().len().min(4) as u16 + 2
    } else {
        0
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(41),
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(suggestion_height),
            Constraint::Length(3),
        ])
        .split(area);

    let mut top_lines = app.dash.clone();
    if let Some(update_banner) = &app.update_banner {
        top_lines.push(Line::from("·"));
        top_lines.push(Line::from(Span::styled(
            update_banner.clone(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }
    top_lines.extend([
        Line::from("·"),
        Line::from(vec![Span::styled(
            app.auth_status.clone(),
            Style::default()
                .fg(status_color(&app.auth_status))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("·"),
        Line::from(vec![Span::styled(
            app.service_status.clone(),
            Style::default()
                .fg(status_color(&app.service_status))
                .add_modifier(Modifier::BOLD),
        )]),
    ]);

    let top = Paragraph::new(top_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(top, chunks[0]);

    let output_items: Vec<ListItem> = if app.networks.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            " no known networks found",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        app.networks
            .iter()
            .flat_map(|network| {
                [
                    ListItem::new(Line::from(Span::styled(
                        format!("  {}", network.name),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ))),
                    ListItem::new(Line::from(Span::styled(
                        format!("  {}", network.host),
                        Style::default().fg(Color::DarkGray),
                    ))),
                    ListItem::new(Line::from("")),
                ]
            })
            .collect()
    };

    let output = List::new(output_items)
        .block(
            Block::default()
                .title_top(Line::from("known_networks").centered())
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(0, 40, 50))
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    let mut output_state = ListState::default();
    output_state.select(app.selected_network.map(|i| i * 3));
    f.render_stateful_widget(output, chunks[1], &mut output_state);

    let input_widget = Paragraph::new(app.input.clone())
        .block(Block::default().title_top("command").borders(Borders::ALL));
    f.render_widget(input_widget, chunks[2]);

    if app.show_suggestions {
        f.set_cursor_position((chunks[2].x + 1 + app.input.len() as u16, chunks[2].y + 1));
    }

    if app.show_suggestions {
        let filtered = app.filtered_suggestions();
        if !filtered.is_empty() {
            let popup_area = Rect {
                x: chunks[2].x,
                y: chunks[3].y,
                width: chunks[2].width,
                height: suggestion_height.min(chunks[3].height),
            };

            let selected = app.suggestion_index.unwrap_or(0);
            let max_visible = 4;
            let start = selected.saturating_sub(max_visible - 1);
            let visible = filtered.iter().skip(start).take(max_visible);

            let items: Vec<ListItem> = visible
                .enumerate()
                .map(|(visible_index, (cmd, desc))| {
                    let is_selected = selected == start + visible_index;
                    let cmd_style = if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(format!(" {:<20}", cmd), cmd_style),
                        Span::styled(desc.to_string(), Style::default().fg(Color::DarkGray)),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::Rgb(0, 40, 50))
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );

            let mut state = ListState::default();
            state.select(app.suggestion_index.map(|i| i.saturating_sub(start)));
            f.render_stateful_widget(list, popup_area, &mut state);
        }
    }

    let helper = Paragraph::new(format!(
        "j/k select · r refetch · / command · Ctrl-c quit · {}",
        app.status
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(helper, chunks[4]);
}

fn status_color(status: &str) -> Color {
    let status = status.to_lowercase();
    if status.contains("unavailable") || status.contains("error") || status.contains("down") {
        Color::Red
    } else if status.contains("unknown") || status.contains("no output") {
        Color::Yellow
    } else {
        Color::Green
    }
}

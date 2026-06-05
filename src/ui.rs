use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, LoginPopup};

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
    ]);
    for detail in &app.auth_details {
        top_lines.push(Line::from(Span::styled(
            detail.clone(),
            Style::default().fg(Color::DarkGray),
        )));
    }
    top_lines.extend([
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

    let output_items: Vec<ListItem> = if app.accounts.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            " no Pangolin accounts found · use /login",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        app.accounts
            .iter()
            .flat_map(|account| {
                let marker = if account.active { "*" } else { " " };
                let label = if account.email.is_empty() {
                    account.user_id.clone()
                } else {
                    account.email.clone()
                };
                [
                    ListItem::new(Line::from(Span::styled(
                        format!(" {marker} {label}"),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ))),
                    ListItem::new(Line::from(Span::styled(
                        format!("   {} · {}", account.host, account.org_id),
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
                .title_top(Line::from("pangolin_accounts").centered())
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(0, 40, 50))
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    let mut output_state = ListState::default();
    output_state.select(app.selected_account.map(|i| i * 3));
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
        "j/k/↑/↓ move · Enter/l select · r refetch · / commands · Ctrl-c quit · {}",
        app.status
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(helper, chunks[4]);

    render_login_popup(f, app, area);
}

fn render_login_popup(f: &mut Frame, app: &App, area: Rect) {
    match &app.login_popup {
        LoginPopup::Hidden => {}
        LoginPopup::Hosting { selected } => {
            let popup = centered_rect(area, 62, 8);
            f.render_widget(Clear, popup);

            let items = [
                ("Pangolin Cloud", "app.pangolin.net"),
                ("Self Hosted", "enter your Pangolin host URL"),
            ]
            .into_iter()
            .enumerate()
            .map(|(index, (label, detail))| {
                let style = if *selected == index {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {:<18}", label), style),
                    Span::styled(detail, Style::default().fg(Color::DarkGray)),
                ]))
            })
            .collect::<Vec<_>>();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title_top(Line::from("pangolin login").centered())
                        .title_bottom(
                            Line::from("Enter choose · Esc cancel · j/k or ↑/↓ move").centered(),
                        )
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Rgb(0, 40, 50))
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );
            let mut state = ListState::default();
            state.select(Some(*selected));
            f.render_stateful_widget(list, popup, &mut state);
        }
        LoginPopup::SelfHosted { host } => {
            let popup = centered_rect(area, 62, 7);
            f.render_widget(Clear, popup);

            let content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Host URL ", Style::default().fg(Color::DarkGray)),
                    Span::styled(host.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
            ];
            let paragraph = Paragraph::new(content).block(
                Block::default()
                    .title_top(Line::from("self-hosted pangolin login").centered())
                    .title_bottom(Line::from("Enter login · Esc cancel").centered())
                    .borders(Borders::ALL),
            );
            f.render_widget(paragraph, popup);
            f.set_cursor_position((popup.x + 11 + host.len() as u16, popup.y + 2));
        }
    }
}

fn centered_rect(area: Rect, width_percent: u16, height: u16) -> Rect {
    let width = area.width.saturating_mul(width_percent).saturating_div(100);
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;

    Rect {
        x,
        y,
        width,
        height: height.min(area.height),
    }
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

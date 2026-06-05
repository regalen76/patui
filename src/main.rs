use std::{error::Error, io};

use libsql::Connection;
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};

use crate::{app::App, ui::ui};

mod app;
mod db;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = db::open_database().await?;
    let conn = database.connect()?;
    db::migrate(&conn).await?;

    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, event::EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    refresh_known_networks(&mut app, &conn).await;

    let res = run_app(&mut terminal, &mut app, &conn).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn run_app<B>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    conn: &Connection,
) -> io::Result<bool>
where
    B: Backend,
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    return Ok(true);
                }

                if app.show_suggestions {
                    match key.code {
                        KeyCode::Down | KeyCode::Char('n')
                            if key.modifiers == KeyModifiers::CONTROL =>
                        {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index =
                                    Some(app.suggestion_index.map_or(0, |i| (i + 1) % len));
                            }
                            continue;
                        }
                        KeyCode::Up | KeyCode::Char('p')
                            if key.modifiers == KeyModifiers::CONTROL =>
                        {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index = Some(
                                    app.suggestion_index
                                        .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 }),
                                );
                            }
                            continue;
                        }
                        KeyCode::Tab | KeyCode::Enter => {
                            if let Some(idx) = app.suggestion_index {
                                if let Some((cmd, _)) = app.filtered_suggestions().get(idx) {
                                    app.input = cmd.to_string();
                                }
                            }
                            app.show_suggestions = false;
                            app.suggestion_index = None;
                            continue;
                        }
                        _ => {}
                    }
                }

                match key.code {
                    KeyCode::Char('/') if app.input.is_empty() => {
                        app.input.push('/');
                        app.show_suggestions = true;
                        app.suggestion_index = Some(0);
                    }
                    KeyCode::Char(value)
                        if key.modifiers.difference(KeyModifiers::SHIFT).is_empty() =>
                    {
                        app.input.push(value);
                        if app.input.starts_with('/') {
                            let filtered = app.filtered_suggestions();
                            app.show_suggestions = !filtered.is_empty();
                            app.suggestion_index = if filtered.is_empty() { None } else { Some(0) };
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        if app.input.starts_with('/') {
                            let filtered = app.filtered_suggestions();
                            app.show_suggestions = !filtered.is_empty();
                            app.suggestion_index = if filtered.is_empty() { None } else { Some(0) };
                        } else {
                            app.show_suggestions = false;
                            app.suggestion_index = None;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous_network(),
                    KeyCode::Down | KeyCode::Char('j') => app.select_next_network(),
                    KeyCode::Char('r') => refresh_known_networks(app, conn).await,
                    KeyCode::Enter => {
                        if app.input.trim() == "/quit" {
                            return Ok(true);
                        }
                        if app.input.trim() == "/refetch" {
                            refresh_known_networks(app, conn).await;
                        }
                        app.input.clear();
                        app.show_suggestions = false;
                        app.suggestion_index = None;
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn refresh_known_networks(app: &mut App, conn: &Connection) {
    match db::known_networks::get_all(conn).await {
        Ok(networks) => app.refresh_networks(networks),
        Err(err) => app.status = format!("DB error: {err}"),
    }
}

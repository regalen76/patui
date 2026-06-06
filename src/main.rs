use std::{error::Error, io, process::Command};

use ratatui::{
    Terminal,
    crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        execute,
        terminal::{
            Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
            enable_raw_mode,
        },
    },
    prelude::{Backend, CrosstermBackend, Rect},
};

use crate::{
    app::{App, LoginPopup},
    ui::ui,
};

mod app;
mod pangolin_accounts;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, event::EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    refresh_pangolin_statuses(&mut app);
    refresh_pangolin_accounts(&mut app);

    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    B: Backend + io::Write,
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

                if handle_login_popup_key(terminal, app, key.code)? {
                    continue;
                }

                if app.show_suggestions {
                    match key.code {
                        KeyCode::Down if key.modifiers == KeyModifiers::NONE => {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index =
                                    Some(app.suggestion_index.map_or(0, |i| (i + 1) % len));
                            }
                            continue;
                        }
                        KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index =
                                    Some(app.suggestion_index.map_or(0, |i| (i + 1) % len));
                            }
                            continue;
                        }
                        KeyCode::Up if key.modifiers == KeyModifiers::NONE => {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index = Some(
                                    app.suggestion_index
                                        .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 }),
                                );
                            }
                            continue;
                        }
                        KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => {
                            let len = app.filtered_suggestions().len();
                            if len > 0 {
                                app.suggestion_index = Some(
                                    app.suggestion_index
                                        .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 }),
                                );
                            }
                            continue;
                        }
                        KeyCode::Tab => {
                            if let Some(idx) = app.suggestion_index {
                                if let Some((cmd, _)) = app.filtered_suggestions().get(idx) {
                                    app.input = cmd.to_string();
                                }
                            }
                            app.show_suggestions = false;
                            app.suggestion_index = None;
                            continue;
                        }
                        KeyCode::Enter => {
                            if let Some(idx) = app.suggestion_index {
                                if let Some((cmd, _)) = app.filtered_suggestions().get(idx) {
                                    app.input = cmd.to_string();
                                }
                            }
                            app.show_suggestions = false;
                            app.suggestion_index = None;
                            if execute_command(terminal, app)? {
                                return Ok(true);
                            }
                            continue;
                        }
                        KeyCode::Esc => {
                            app.input.clear();
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
                        if app.input.starts_with('/')
                            && key.modifiers.difference(KeyModifiers::SHIFT).is_empty() =>
                    {
                        app.input.push(value);
                        if app.input.starts_with('/') {
                            let filtered = app.filtered_suggestions();
                            app.show_suggestions = !filtered.is_empty();
                            app.suggestion_index = if filtered.is_empty() { None } else { Some(0) };
                        }
                    }
                    KeyCode::Backspace if app.input.starts_with('/') => {
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
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous_account(),
                    KeyCode::Down | KeyCode::Char('j') => app.select_next_account(),
                    KeyCode::Char('r') => refresh_all(app),
                    KeyCode::Char('l') => {
                        app.connecting_popup = Some(String::from("Please wait..."));
                        terminal.draw(|f| ui(f, app))?;
                        select_active_account(app);
                    }
                    KeyCode::Enter if app.input.starts_with('/') => {
                        if execute_command(terminal, app)? {
                            return Ok(true);
                        }
                    }
                    KeyCode::Enter => {
                        app.connecting_popup = Some(String::from("Please wait..."));
                        terminal.draw(|f| ui(f, app))?;
                        select_active_account(app);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn execute_command<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    B: Backend + io::Write,
    io::Error: From<B::Error>,
{
    let input = app.input.trim().to_string();
    let mut should_quit = false;

    if input == "/quit" {
        should_quit = true;
    } else if input == "/refetch" {
        refresh_all(app);
    } else if input == "/login" {
        app.login_popup = LoginPopup::Hosting { selected: 0 };
    } else if input == "/logout" {
        if app.service_status.contains("Connected") {
            app.login_popup = LoginPopup::LogoutConfirm { selected: 0 };
        } else {
            run_pangolin_logout(app);
        }
    } else if input == "/up" {
        run_pangolin_service(terminal, app, "up")?;
    } else if input == "/down" {
        run_pangolin_service(terminal, app, "down")?;
    } else if let Some(host) = input.strip_prefix("/login ") {
        run_pangolin_login(terminal, app, host.trim())?;
    }

    app.input.clear();
    app.show_suggestions = false;
    app.suggestion_index = None;

    Ok(should_quit)
}

fn handle_login_popup_key<B>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    code: KeyCode,
) -> io::Result<bool>
where
    B: Backend + io::Write,
    io::Error: From<B::Error>,
{
    let mut login_host = None;

    match &mut app.login_popup {
        LoginPopup::Hidden => return Ok(false),
        LoginPopup::Hosting { selected } => match code {
            KeyCode::Esc => app.login_popup = LoginPopup::Hidden,
            KeyCode::Up | KeyCode::Char('k') => *selected = 0,
            KeyCode::Down | KeyCode::Char('j') => *selected = 1,
            KeyCode::Enter => {
                if *selected == 0 {
                    app.login_popup = LoginPopup::Hidden;
                    login_host = Some(String::from("https://app.pangolin.net"));
                } else {
                    app.login_popup = LoginPopup::SelfHosted {
                        host: String::new(),
                    };
                }
            }
            _ => {}
        },
        LoginPopup::SelfHosted { host } => match code {
            KeyCode::Esc => app.login_popup = LoginPopup::Hidden,
            KeyCode::Char(value) => host.push(value),
            KeyCode::Backspace => {
                host.pop();
            }
            KeyCode::Enter => {
                let input_host = host.trim().to_string();
                if input_host.is_empty() {
                    app.status = String::from("Enter self-hosted Pangolin host URL");
                } else {
                    app.login_popup = LoginPopup::Hidden;
                    login_host = Some(input_host);
                }
            }
            _ => {}
        },
        LoginPopup::LogoutConfirm { selected } => {
            let do_logout = *selected == 0;
            match code {
                KeyCode::Esc => app.login_popup = LoginPopup::Hidden,
                KeyCode::Left | KeyCode::Char('h') => *selected = 0,
                KeyCode::Right | KeyCode::Char('l') => *selected = 1,
                KeyCode::Enter => {
                    app.login_popup = LoginPopup::Hidden;
                    if do_logout {
                        run_pangolin_service(terminal, app, "down")?;
                        run_pangolin_logout(app);
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(host) = login_host {
        run_pangolin_login(terminal, app, &host)?;
    }

    Ok(true)
}

fn refresh_all(app: &mut App) {
    refresh_pangolin_statuses(app);
    refresh_pangolin_accounts(app);
    app.connecting_popup = None;
}

fn refresh_pangolin_accounts(app: &mut App) {
    match pangolin_accounts::load_accounts() {
        Ok(accounts) => app.refresh_accounts(accounts),
        Err(err) => app.status = format!("Pangolin accounts error: {err}"),
    }
}

fn run_pangolin_login<B>(terminal: &mut Terminal<B>, app: &mut App, host: &str) -> io::Result<()>
where
    B: Backend + io::Write,
    io::Error: From<B::Error>,
{
    if host.is_empty() {
        app.status = String::from("Usage: /login https://your-instance.example.com");
        return Ok(());
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    println!("Running pangolin login {host}");

    let login_status = Command::new("pangolin").args(["login", host]).status();

    restore_tui(terminal, app)?;

    match login_status {
        Ok(status) if status.success() => {
            app.status = String::from("Pangolin login completed");
            refresh_all(app);
        }
        Ok(status) => app.status = format!("Pangolin login failed: {status}"),
        Err(err) => app.status = format!("Pangolin login unavailable: {err}"),
    }

    Ok(())
}

fn restore_tui<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B: Backend + io::Write,
    io::Error: From<B::Error>,
{
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        event::EnableMouseCapture,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    terminal.clear()?;
    terminal.resize(Rect::new(0, 0, 0, 0))?;
    terminal.draw(|f| ui(f, app))?;

    Ok(())
}

fn run_pangolin_logout(app: &mut App) {
    app.connecting_popup = Some(String::from("Please wait..."));
    match Command::new("pangolin").arg("logout").output() {
        Ok(output) if output.status.success() => {
            let message = parse_command_message(&output.stdout, &output.stderr);
            app.status = if message.is_empty() {
                String::from("Pangolin logout completed")
            } else {
                message
            };
            app.connecting_popup = None;
            refresh_all(app);
        }
        Ok(output) => {
            app.connecting_popup = None;
            let message = parse_command_message(&output.stderr, &output.stdout);
            app.status = format!("Pangolin logout failed: {message}");
        }
        Err(err) => {
            app.connecting_popup = None;
            app.status = format!("Pangolin logout unavailable: {err}");
        }
    }
}

fn run_pangolin_service<B>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    command: &str,
) -> io::Result<()>
where
    B: Backend + io::Write,
    io::Error: From<B::Error>,
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    println!("Running pangolin {command}");

    let service_status = Command::new("pangolin").arg(command).status();

    restore_tui(terminal, app)?;

    match service_status {
        Ok(status) if status.success() => {
            app.status = format!("Pangolin {command} completed");
            refresh_all(app);
        }
        Ok(status) => app.status = format!("Pangolin {command} failed: {status}"),
        Err(err) => app.status = format!("Pangolin {command} unavailable: {err}"),
    }

    Ok(())
}

fn select_active_account(app: &mut App) {
    let Some(index) = app.selected_account else {
        app.connecting_popup = None;
        app.status = String::from("No Pangolin account selected");
        return;
    };
    let Some(account) = app.accounts.get(index) else {
        app.connecting_popup = None;
        app.status = String::from("No Pangolin account selected");
        return;
    };

    match Command::new("pangolin")
        .args([
            "select",
            "account",
            "--account",
            &account.email,
            "--host",
            &account.host,
        ])
        .output()
    {
        Ok(output) if output.status.success() => {
            app.status = format!("Selected {} @ {}", account.email, account.host);
            refresh_all(app);
        }
        Ok(output) => {
            app.connecting_popup = None;
            let message = parse_command_message(&output.stderr, &output.stdout);
            app.status = format!("Select account failed: {message}");
        }
        Err(err) => {
            app.connecting_popup = None;
            app.status = format!("Select account unavailable: {err}");
        }
    }
}

fn refresh_pangolin_statuses(app: &mut App) {
    if !pangolin_cli_available() {
        app.update_banner = None;
        app.auth_status = String::from("Pangolin CLI not found in PATH");
        app.auth_details = vec![String::from(
            "Install Pangolin CLI and ensure `pangolin` is on PATH",
        )];
        app.service_status = String::from("Pangolin features unavailable");
        app.status = String::from("Pangolin CLI required");
        return;
    }

    let auth_output = pangolin_output(["auth", "status"]);
    let service_output = pangolin_output(["status"]);

    app.update_banner = auth_output
        .update_banner
        .clone()
        .or_else(|| service_output.update_banner.clone());
    app.auth_status = auth_output.status;
    app.auth_details = auth_output.details;
    app.service_status = service_output.status;
}

fn pangolin_cli_available() -> bool {
    Command::new("pangolin").arg("--version").output().is_ok()
}

struct PangolinOutput {
    update_banner: Option<String>,
    status: String,
    details: Vec<String>,
}

fn pangolin_output<const N: usize>(args: [&str; N]) -> PangolinOutput {
    match Command::new("pangolin").args(args).output() {
        Ok(output) if output.status.success() => parse_pangolin_output(&output.stdout),
        Ok(output) => parse_pangolin_output(&output.stderr),
        Err(err) => PangolinOutput {
            update_banner: None,
            status: format!("unavailable ({err})"),
            details: Vec::new(),
        },
    }
}

fn parse_pangolin_output(bytes: &[u8]) -> PangolinOutput {
    let output = String::from_utf8_lossy(bytes);
    let lines = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let update_banner = lines
        .iter()
        .find(|line| line.starts_with("A new version is available:"))
        .map(|line| (*line).to_string());
    let status_lines = lines
        .iter()
        .filter(|line| !line.starts_with("A new version is available:"))
        .filter(|line| !line.starts_with("Run 'pangolin update'"))
        .filter(|line| !line.starts_with("Community Edition."))
        .map(|line| (*line).to_string())
        .collect::<Vec<_>>();
    let details = status_lines
        .iter()
        .filter(|line| {
            line.starts_with("User:") || line.starts_with("User ID:") || line.starts_with("Org ID:")
        })
        .cloned()
        .collect::<Vec<_>>();

    if let Some(status) = status_lines.iter().find(|line| line.starts_with("Status:")) {
        let status = if let Some(host) = status_lines.iter().find(|line| line.starts_with('@')) {
            format!("{} {}", status.trim_start_matches("Status: "), host)
        } else {
            status.trim_start_matches("Status: ").to_string()
        };
        return PangolinOutput {
            update_banner,
            status,
            details,
        };
    }

    PangolinOutput {
        update_banner,
        status: status_lines
            .iter()
            .find(|line| !is_pangolin_table_header(line))
            .cloned()
            .unwrap_or_else(|| String::from("no output")),
        details,
    }
}

fn is_pangolin_table_header(line: &str) -> bool {
    line.starts_with("AGENT") || line.starts_with("SITE")
}

fn parse_command_message(stderr: &[u8], stdout: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stdout = String::from_utf8_lossy(stdout);
    stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("no output")
        .to_string()
}

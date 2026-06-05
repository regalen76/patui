use crate::pangolin_accounts::PangolinAccount;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub struct App {
    pub dash: Vec<Line<'static>>,
    pub update_banner: Option<String>,
    pub auth_status: String,
    pub auth_details: Vec<String>,
    pub service_status: String,
    pub accounts: Vec<PangolinAccount>,
    pub selected_account: Option<usize>,
    pub input: String,
    pub suggestions: Vec<(&'static str, &'static str)>,
    pub suggestion_index: Option<usize>,
    pub show_suggestions: bool,
    pub login_popup: LoginPopup,
    pub connecting_popup: Option<String>,
    pub status: String,
}

pub enum LoginPopup {
    Hidden,
    Hosting { selected: usize },
    SelfHosted { host: String },
}

impl App {
    pub fn new() -> App {
        App {
            dash: create_dashboard(),
            update_banner: None,
            auth_status: String::from("unknown"),
            auth_details: Vec::new(),
            service_status: String::from("unknown"),
            accounts: Vec::new(),
            selected_account: None,
            input: String::new(),
            suggestions: vec![
                ("/quit", "exit the application"),
                ("/refetch", "refresh Pangolin statuses and accounts"),
                ("/login", "open Pangolin login chooser"),
                ("/logout", "logout active Pangolin account"),
                ("/up", "start Pangolin connection"),
                ("/down", "stop Pangolin connection"),
                ("/help", "show help"),
                ("/clear", "clear output"),
            ],
            suggestion_index: None,
            show_suggestions: false,
            login_popup: LoginPopup::Hidden,
            connecting_popup: None,
            status: String::from("Pangolin CLI required"),
        }
    }

    pub fn filtered_suggestions(&self) -> Vec<(&'static str, &'static str)> {
        self.suggestions
            .iter()
            .filter(|(cmd, _)| cmd.starts_with(&self.input))
            .copied()
            .collect()
    }

    pub fn refresh_accounts(&mut self, accounts: Vec<PangolinAccount>) {
        self.accounts = accounts;
        self.selected_account = if self.accounts.is_empty() {
            None
        } else {
            Some(
                self.selected_account
                    .unwrap_or(0)
                    .min(self.accounts.len() - 1),
            )
        };
        self.status = format!("{} Pangolin accounts", self.accounts.len());
    }

    pub fn select_next_account(&mut self) {
        let len = self.accounts.len();
        if len > 0 {
            self.selected_account = Some(self.selected_account.map_or(0, |i| (i + 1) % len));
        }
    }

    pub fn select_previous_account(&mut self) {
        let len = self.accounts.len();
        if len > 0 {
            self.selected_account = Some(
                self.selected_account
                    .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 }),
            );
        }
    }
}

fn create_dashboard() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "██████╗     ████████╗██╗   ██╗██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔══██╗    ╚══██╔══╝██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██████╔╝       ██║   ██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔═══╝        ██║   ██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║      ██╗   ██║   ╚██████╔╝██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚═╝      ╚═╝   ╚═╝    ╚═════╝ ╚═╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " · PA.TUI · ",
            Style::default().fg(Color::DarkGray),
        )]),
    ]
}

use crate::db::known_networks::KnownNetwork;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub struct App {
    pub dash: Vec<Line<'static>>,
    pub networks: Vec<KnownNetwork>,
    pub selected_network: Option<usize>,
    pub input: String,
    pub suggestions: Vec<(&'static str, &'static str)>,
    pub suggestion_index: Option<usize>,
    pub show_suggestions: bool,
    pub status: String,
}

impl App {
    pub fn new() -> App {
        App {
            dash: create_dashboard(),
            networks: Vec::new(),
            selected_network: None,
            input: String::new(),
            suggestions: vec![
                ("/quit", "exit the application"),
                ("/refetch", "refresh known networks"),
                ("/connect", "connect to selected Pangolin network"),
                ("/help", "show help"),
                ("/clear", "clear output"),
            ],
            suggestion_index: None,
            show_suggestions: false,
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

    pub fn refresh_networks(&mut self, networks: Vec<KnownNetwork>) {
        self.networks = networks;
        self.selected_network = if self.networks.is_empty() {
            None
        } else {
            Some(
                self.selected_network
                    .unwrap_or(0)
                    .min(self.networks.len() - 1),
            )
        };
        self.status = format!("{} known networks", self.networks.len());
    }

    pub fn select_next_network(&mut self) {
        let len = self.networks.len();
        if len > 0 {
            self.selected_network = Some(self.selected_network.map_or(0, |i| (i + 1) % len));
        }
    }

    pub fn select_previous_network(&mut self) {
        let len = self.networks.len();
        if len > 0 {
            self.selected_network = Some(
                self.selected_network
                    .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 }),
            );
        }
    }
}

fn create_dashboard() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "██████╗     ██╗   ██╗██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔══██╗    ██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██████╔╝    ██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔═══╝     ██║   ██║██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║      ██╗╚██████╔╝██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚═╝      ╚═╝ ╚═════╝ ╚═╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " · Pangolin UI · ",
            Style::default().fg(Color::DarkGray),
        )]),
    ]
}

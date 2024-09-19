use crate::game_state::GameState;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdout, Stdout};
use std::time::Duration;

pub enum Action {
    Generate,
    Customize,
    Save,
    Load,
    Quit,
}

pub struct UI {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    loading_texts: Vec<String>,
    current_loading_text: usize,
    current_spinner_char: char,
}

impl UI {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        let loading_texts = vec![
            "nangis karena disakitin ayang".to_string(),
            "sedih gak diajak telponan sama ayang".to_string(),
            "galau kangen ayang".to_string(),
            "ayang kemana sih, kok gak nyuruh aku makan".to_string(),
            "ngambek belum disapa ayang".to_string(),
            "malu dibikinin story ayang".to_string(),
            "betmut belum dichat ayang".to_string(),
            "love u ayang".to_string(),
            "sedih ayangnya fiksi".to_string(),
            "pusing gapunya ayang".to_string(),
            "udah dapat banyak pap ayang".to_string(),
            "sedang mencari ayang".to_string(),
        ];

        Ok(UI {
            terminal,
            loading_texts,
            current_loading_text: 0,
            current_spinner_char: '|',
        })
    }

    pub fn display(&mut self, game_state: &GameState) -> Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let title = Paragraph::new("Random Adventure Generator")
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(ratatui::layout::Alignment::Center);

            f.render_widget(title, chunks[0]);

            let adventure_text = match &game_state.current_adventure {
                Some(adventure) => adventure.to_string(),
                None => {
                    "No adventure generated yet. Press 'g' to generate a new adventure".to_string()
                }
            };

            let adveture_paragraph = Paragraph::new(adventure_text)
                .block(
                    Block::default()
                        .title("Current Adventure")
                        .borders(Borders::ALL),
                )
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(adveture_paragraph, chunks[1]);

            let controls = Line::from(vec![
                Span::raw("Press "),
                Span::styled("g", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to generate, "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to customize, "),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to save, "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to load, "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit"),
            ]);
            let controls_paragraph = Paragraph::new(controls)
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);

            f.render_widget(controls_paragraph, chunks[2]);
        })?;

        Ok(())
    }

    pub fn update_spinner(&mut self, spinner_char: char) -> Result<()> {
        self.current_spinner_char = spinner_char;
        self.display_loading()
    }

    pub fn update_loading_text(&mut self) -> Result<()> {
        self.current_loading_text = (self.current_loading_text + 1) % self.loading_texts.len();
        self.display_loading()
    }

    fn display_loading(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let title = Paragraph::new("Random Adventure Generator")
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);

            let spinner_text = format!("Generating adventure... {}", self.current_spinner_char);
            let spinner_paragraph = Paragraph::new(spinner_text)
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(spinner_paragraph, chunks[1]);

            let loading_text = &self.loading_texts[self.current_loading_text];
            let loading_paragraph = Paragraph::new(format!("\"{}\"", loading_text))
                .style(Style::default().fg(Color::LightCyan))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(loading_paragraph, chunks[2]);
        })?;

        Ok(())
    }

    pub fn handle_input(&self) -> Result<Option<Action>> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('g') => Ok(Some(Action::Generate)),
                    KeyCode::Char('c') => Ok(Some(Action::Customize)),
                    KeyCode::Char('s') => Ok(Some(Action::Save)),
                    KeyCode::Char('l') => Ok(Some(Action::Load)),
                    KeyCode::Char('q') => Ok(Some(Action::Quit)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

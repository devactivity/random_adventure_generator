use crate::adventure::Adventure;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub settings: AdventureSettings,
    pub current_adventure: Option<Adventure>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AdventureSettings {
    pub difficulty: Difficulty,
    pub genre: Genre,
    pub length: Length,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Genre {
    Fantasy,
    SciFi,
    Horror,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Length {
    Short,
    Medium,
    Long,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            settings: AdventureSettings {
                difficulty: Difficulty::Medium,
                genre: Genre::Fantasy,
                length: Length::Medium,
            },
            current_adventure: None,
        }
    }

    pub fn set_current_adventure(&mut self, adventure: Adventure) {
        self.current_adventure = Some(adventure);
    }

    pub fn customize_settings<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut selected_option = 0;
        let options = vec!["Difficulty", "Genre", "Length", "Save and Exit"];

        loop {
            terminal.draw(|f| self.draw_customize_menu(f, &options, selected_option))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        selected_option = selected_option.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        if selected_option < options.len() - 1 {
                            selected_option += 1;
                        }
                    }
                    KeyCode::Enter => match selected_option {
                        0 => self.cycle_difficulty(),
                        1 => self.cycle_genre(),
                        2 => self.cycle_length(),
                        3 => return Ok(()),
                        _ => {}
                    },
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }

    fn draw_customize_menu(&self, f: &mut Frame, options: &[&str], selected_option: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.area());

        let title = Paragraph::new("Customize Adventure Settings")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, &option)| {
                let content = match i {
                    0 => format!("{}: {:?}", option, self.settings.difficulty),
                    1 => format!("{}: {:?}", option, self.settings.genre),
                    2 => format!("{}: {:?}", option, self.settings.length),
                    _ => option.to_string(),
                };
                let style = if i == selected_option {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(vec![Span::styled(content, style)]))
            })
            .collect();

        let options_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Options"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(
            options_list,
            chunks[1],
            &mut ratatui::widgets::ListState::default(),
        );

        let instructions =
            Paragraph::new("Use Up/Down arrows to navigate, Enter to select, esc to exit")
                .style(Style::default().fg(Color::Gray))
                .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(instructions, chunks[2]);
    }

    fn cycle_difficulty(&mut self) {
        self.settings.difficulty = match self.settings.difficulty {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        };
    }

    fn cycle_genre(&mut self) {
        self.settings.genre = match self.settings.genre {
            Genre::Fantasy => Genre::SciFi,
            Genre::SciFi => Genre::Horror,
            Genre::Horror => Genre::Fantasy,
        };
    }

    fn cycle_length(&mut self) {
        self.settings.length = match self.settings.length {
            Length::Short => Length::Medium,
            Length::Medium => Length::Long,
            Length::Long => Length::Short,
        };
    }

    pub fn save_adventure(&self) -> Result<()> {
        if let Some(adventure) = &self.current_adventure {
            let serialized = serde_json::to_string(adventure)?;
            std::fs::write("saved_adventure.json", serialized)?;
        }

        Ok(())
    }

    pub fn load_adventure(&mut self) -> Result<()> {
        let serialized = std::fs::read_to_string("saved_adventure.json")?;
        let adventure: Adventure = serde_json::from_str(&serialized)?;

        self.current_adventure = Some(adventure);
        Ok(())
    }
}

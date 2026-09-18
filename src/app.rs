use std::{io, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::{
    event,
    sections::{self, Section},
};

const POLL_INTERVAL: Duration = Duration::from_millis(250);

pub struct App {
    sections: Vec<Box<dyn Section>>,
    active_section: usize,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            sections: sections::default_sections(),
            active_section: 0,
            running: true,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(event) = event::next(POLL_INTERVAL)? {
                self.handle_event(event);
            }

            if self.running {
                self.active_section_mut().update();
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        let Event::Key(key) = event else {
            return;
        };

        if key.kind != KeyEventKind::Press {
            return;
        }

        if self.handle_global_key(key) {
            return;
        }

        self.active_section_mut().handle_event(&Event::Key(key));
    }

    fn handle_global_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                true
            }
            KeyCode::Tab => {
                self.next_section();
                true
            }
            KeyCode::BackTab => {
                self.previous_section();
                true
            }
            KeyCode::Char(number @ '1'..='9') => {
                let index = (number as u8 - b'1') as usize;
                if index < self.sections.len() {
                    self.active_section = index;
                }
                true
            }
            _ => false,
        }
    }

    fn next_section(&mut self) {
        self.active_section = (self.active_section + 1) % self.sections.len();
    }

    fn previous_section(&mut self) {
        self.active_section = (self.active_section + self.sections.len() - 1) % self.sections.len();
    }

    fn active_section_mut(&mut self) -> &mut dyn Section {
        self.sections[self.active_section].as_mut()
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let titles: Vec<Line> = self
            .sections
            .iter()
            .enumerate()
            .map(|(index, section)| Line::from(format!("[{}] {}", index + 1, section.name())))
            .collect();
        frame.render_widget(
            Tabs::new(titles)
                .select(self.active_section)
                .highlight_style(Style::default().fg(Color::Cyan))
                .divider("  ")
                .block(Block::bordered().title(Span::styled(
                    "Life Terminal",
                    Style::default().add_modifier(Modifier::BOLD)
                ))),
            layout[1],
        );

        self.active_section_mut().render(frame, layout[2]);

        frame.render_widget(
            Paragraph::new("Tab: Next   Shift+Tab: Previous   1-6: Switch section   q: Quit")
                .block(Block::default().borders(Borders::ALL)),
            layout[3],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn navigation_wraps_between_sections() {
        let mut app = App::new();
        app.previous_section();
        assert_eq!(app.active_section, 5);
        app.next_section();
        assert_eq!(app.active_section, 0);
    }
}

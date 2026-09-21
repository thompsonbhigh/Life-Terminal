use std::{io, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::{
    db::Database,
    event,
    sections::{self, Section},
};

const POLL_INTERVAL: Duration = Duration::from_millis(250);

pub struct App {
    database: Database,
    sections: Vec<Box<dyn Section>>,
    active_section: usize,
    running: bool,
}

impl App {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            sections: sections::default_sections(),
            active_section: 0,
            running: true,
        }
    }

    pub fn run<B: Backend<Error = io::Error>>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
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

        if !self.sections[self.active_section].captures_input() && self.handle_global_key(key) {
            return;
        }

        self.sections[self.active_section].handle_event(&Event::Key(key), &self.database);
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
        let task_count = self.database.list_tasks().map_or(0, |tasks| tasks.len());
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
                    Style::default().add_modifier(Modifier::BOLD),
                ))),
            layout[1],
        );

        let active_section = self.active_section;
        let database = &self.database;

        self.sections[active_section].render(frame, layout[2], database);

        frame.render_widget(
            Paragraph::new(if self.sections[active_section].captures_input() {
                "   Enter: Save task   Esc: Cancel"
            } else {
                "   Tab: Next   Shift+Tab: Previous   1-6: Switch section   q: Quit"
            })
            .block(Block::default().borders(Borders::ALL)),
            layout[3],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::db::Database;

    #[test]
    fn navigation_wraps_between_sections() {
        let database = Database::open(":memory:").unwrap();
        let mut app = App::new(database);
        app.previous_section();
        assert_eq!(app.active_section, 5);
        app.next_section();
        assert_eq!(app.active_section, 0);
    }
    #[test]
    fn task_popup_captures_shortcuts_and_saves() {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
        let mut app = App::new(Database::open(":memory:").unwrap());
        let press = |app: &mut App, code| {
            app.handle_event(Event::Key(KeyEvent::new(code, KeyModifiers::NONE)));
        };
        press(&mut app, KeyCode::Char('2'));
        press(&mut app, KeyCode::Char('a'));
        press(&mut app, KeyCode::Char('q'));
        press(&mut app, KeyCode::Char('1'));
        press(&mut app, KeyCode::Tab);
        assert!(app.running);
        assert_eq!(app.active_section, 1);
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.database.list_tasks().unwrap()[0].title, "q1");
        assert!(!app.sections[1].captures_input());
        press(&mut app, KeyCode::Char('q'));
        assert!(!app.running);
    }

    #[test]
    fn task_popup_renders_and_cancels_without_saving() {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
        use ratatui::{backend::TestBackend, Terminal};
        let mut app = App::new(Database::open(":memory:").unwrap());
        app.active_section = 1;
        for code in [KeyCode::Char('a'), KeyCode::Char('x')] {
            app.handle_event(Event::Key(KeyEvent::new(code, KeyModifiers::NONE)));
        }
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| app.render(frame)).unwrap();
        let screen: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect();
        assert!(screen.contains("Add a task"));
        app.handle_event(Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
        assert!(!app.sections[1].captures_input());
        assert!(app.database.list_tasks().unwrap().is_empty());
    }
}

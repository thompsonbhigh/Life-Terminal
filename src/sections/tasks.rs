use crate::db::Database;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
    Frame,
};
use ratatui_textarea::TextArea;

use super::Section;

#[derive(Default)]
pub struct Tasks {
    textarea: Option<TextArea<'static>>,
    error: Option<String>,
}

impl Section for Tasks {
    fn name(&self) -> &'static str {
        "Tasks"
    }

    fn captures_input(&self) -> bool {
        self.textarea.is_some()
    }

    fn handle_event(&mut self, event: &Event, database: &Database) {
        let Event::Key(key) = event else { return };

        if let Some(textarea) = &mut self.textarea {
            match key.code {
                KeyCode::Esc => {
                    self.textarea = None;
                    self.error = None;
                }
                KeyCode::Enter => {
                    let title = textarea.lines().join(" ");
                    let title = title.trim();
                    if title.is_empty() {
                        self.error = Some("Please enter a task title.".into());
                    } else {
                        match database.add_task(title) {
                            Ok(()) => {
                                self.textarea = None;
                                self.error = None;
                            }
                            Err(error) => {
                                self.error = Some(format!("Could not save task: {error}"))
                            }
                        }
                    }
                }
                _ => {
                    textarea.input(*key);
                }
            }
        } else if key.code == KeyCode::Char('a') {
            let mut textarea = TextArea::default();
            textarea.set_block(Block::default()
                .title("Add a task")
                .borders(Borders::ALL)
                .border_style(Style::default().bg(Color::Black))
        );
            textarea.set_placeholder_text("Enter a task");
            textarea.set_cursor_line_style(Style::default());
            self.textarea = Some(textarea);
            self.error = None;
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        let content = match database.list_tasks() {
            Ok(tasks) if tasks.is_empty() => "No tasks yet - press [a] to add one.".to_string(),
            Ok(tasks) => tasks
                .iter()
                .map(|task| {
                    let mark = if task.completed { "✓" } else { "○" };
                    format!("{mark} {}", task.title)
                })
                .collect::<Vec<_>>()
                .join("\n"),
            Err(error) => format!("Could not load tasks: {error}"),
        };

        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(24)])
            .split(area);

        frame.render_widget(
            Paragraph::new(content).block(Block::bordered().title("Tasks")),
            panes[0],
        );

        frame.render_widget(
            Paragraph::new(
                "[a] Add task\n\
               \n\
                [d] Delete task\n\
               \n\
                [Space] Complete\n\
               \n\
                [↑/↓] Select",
            )
            .block(
                Block::bordered()
                    .title("Task Controls")
                    .padding(Padding::proportional(1)),
            ),
            panes[1],
        );

        if let Some(textarea) = &self.textarea {
            let task_area = panes[0];
            let width = area.width.min(60);
            let height = area.height.min(7);
            let popup = Rect::new(
                task_area.x + (task_area.width - width) / 2,
                task_area.y + (task_area.height - height) / 2,
                width,
                height,
            );
            let rows = Layout::vertical([Constraint::Min(3), Constraint::Length(2)]).split(popup);
            frame.render_widget(Clear, popup);
            frame.render_widget(textarea, rows[0]);
            frame.render_widget(
                Paragraph::new(self.error.as_deref().unwrap_or("Enter: Save   Esc: Cancel")).style(
                    Style::default().fg(if self.error.is_some() {
                        Color::Red
                    } else {
                        Color::Gray
                    }),
                ),
                rows[1],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};

    fn press(tasks: &mut Tasks, database: &Database, code: KeyCode) {
        tasks.handle_event(
            &Event::Key(KeyEvent::new(code, KeyModifiers::NONE)),
            database,
        );
    }

    #[test]
    fn empty_title_is_rejected() {
        let database = Database::open(":memory:").unwrap();
        let mut tasks = Tasks::default();
        for code in [KeyCode::Char('a'), KeyCode::Char(' '), KeyCode::Enter] {
            press(&mut tasks, &database, code);
        }
        assert!(tasks.captures_input());
        assert!(tasks.error.is_some());
        assert!(database.list_tasks().unwrap().is_empty());
    }

    #[test]
    fn failed_save_preserves_input() {
        let database = Database::open(":memory:").unwrap();
        let path =
            std::env::temp_dir().join(format!("life-terminal-readonly-{}.db", std::process::id()));
        let setup = rusqlite::Connection::open(&path).unwrap();
        setup.execute_batch("CREATE TABLE tasks (id INTEGER PRIMARY KEY, title TEXT NOT NULL, completed INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP); CREATE TRIGGER reject_task BEFORE INSERT ON tasks BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
        let failing_database = Database::open(path.to_str().unwrap()).unwrap();
        let mut tasks = Tasks::default();
        press(&mut tasks, &database, KeyCode::Char('a'));
        press(&mut tasks, &database, KeyCode::Char('x'));
        press(&mut tasks, &failing_database, KeyCode::Enter);
        assert_eq!(tasks.textarea.as_ref().unwrap().lines(), &["x"]);
        assert!(tasks.error.as_ref().unwrap().contains("test failure"));
        drop(failing_database);
        drop(setup);
        std::fs::remove_file(path).unwrap();
    }
}

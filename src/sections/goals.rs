use crate::db::Database;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use ratatui_textarea::TextArea;

use super::Section;

#[derive(Default)]
pub struct Goals {
    list_state: ListState,
    textarea: Option<TextArea<'static>>,
    error: Option<String>,
}

impl Goals {
    fn sync_selection(&mut self, goal_count: usize) {
        self.list_state.select(if goal_count == 0 {
            None
        } else {
            Some(self.list_state.selected().unwrap_or(0).min(goal_count - 1))
        });
    }
}

impl Section for Goals {
    fn name(&self) -> &'static str {
        "Goals"
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
                        self.error = Some("Please enter a goal title.".into());
                    } else {
                        match database.add_goal(title) {
                            Ok(()) => {
                                self.textarea = None;
                                self.error = None;
                            }
                            Err(error) => {
                                self.error = Some(format!("Could not save goal: {error}"))
                            }
                        }
                    }
                }
                _ => {
                    textarea.input(*key);
                }
            }
        } else if matches!(
            key.code,
            KeyCode::Char('j' | 'k') | KeyCode::Down | KeyCode::Up
        ) {
            if let Ok(goals) = database.list_goals() {
                self.sync_selection(goals.len());
                if let Some(selected) = self.list_state.selected() {
                    let next = match key.code {
                        KeyCode::Char('j') | KeyCode::Down => (selected + 1).min(goals.len() - 1),
                        _ => selected.saturating_sub(1),
                    };
                    self.list_state.select(Some(next));
                }
            }
        } else if key.code == KeyCode::Char('a') {
            let mut textarea = TextArea::default();
            textarea.set_block(
                Block::default()
                    .title("Add a goal")
                    .borders(Borders::ALL)
                    .border_style(Style::default().bg(Color::Black)),
            );
            textarea.set_placeholder_text("Enter a goal");
            textarea.set_cursor_line_style(Style::default());
            self.textarea = Some(textarea);
            self.error = None;
        } else if key.code == KeyCode::Char(' ') {
            if let Ok(goals) = database.list_goals() {
                if let Some(goal) = self
                    .list_state
                    .selected()
                    .and_then(|index| goals.get(index))
                {
                    if let Err(error) = database.toggle_goal(goal.id) {
                        self.error = Some(format!("Could not update goal: {error}"));
                    }
                }
            }
        } else if key.code == KeyCode::Char('d') {
            if let Ok(goals) = database.list_goals() {
                if let Some(goal) = self
                    .list_state
                    .selected()
                    .and_then(|index| goals.get(index))
                {
                    if let Err(error) = database.delete_goal(goal.id) {
                        self.error = Some(format!("Could not delete goal: {error}"));
                    }
                }
            }
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        let goals = database.list_goals();

        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(24)])
            .split(area);

        let block = Block::bordered()
            .title("Goals")
            .padding(Padding::proportional(1));
        match goals {
            Ok(goals) if !goals.is_empty() => {
                self.sync_selection(goals.len());
                let items = goals.iter().map(|goal| {
                    let mark = if goal.completed { "✓" } else { "○" };
                    ListItem::new(format!("{mark} {}", goal.title))
                });
                frame.render_stateful_widget(
                    List::new(items)
                        .block(block)
                        .highlight_style(
                            Style::default()
                                .bg(Color::Cyan)
                                .fg(Color::Black)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol("> "),
                    panes[0],
                    &mut self.list_state,
                );
            }
            result => {
                self.list_state.select(None);
                let message = match result {
                    Ok(_) => "No goals yet - press [a] to add one.".to_string(),
                    Err(error) => format!("Could not load goals: {error}"),
                };
                frame.render_widget(Paragraph::new(message).block(block), panes[0]);
            }
        }

        frame.render_widget(
            Paragraph::new(
                "[a] Add goal\n\
               \n\
                [d] Delete goal\n\
               \n\
                [Space] Complete\n\
               \n\
                [j/↓] Next goal\n\
                \n\
                [k/↑] Previous goal",
            )
            .block(
                Block::bordered()
                    .title("goal Controls")
                    .padding(Padding::proportional(1)),
            ),
            panes[1],
        );

        if let Some(textarea) = &self.textarea {
            let goal_area = panes[0];
            let width = goal_area.width.min(60);
            let height = goal_area.height.min(7);
            let popup = Rect::new(
                goal_area.x + (goal_area.width - width) / 2,
                goal_area.y + (goal_area.height - height) / 2,
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

    fn press(goals: &mut Goals, database: &Database, code: KeyCode) {
        goals.handle_event(
            &Event::Key(KeyEvent::new(code, KeyModifiers::NONE)),
            database,
        );
    }

    #[test]
    fn empty_title_is_rejected() {
        let database = Database::open(":memory:").unwrap();
        let mut goals = Goals::default();
        for code in [KeyCode::Char('a'), KeyCode::Char(' '), KeyCode::Enter] {
            press(&mut goals, &database, code);
        }
        assert!(goals.captures_input());
        assert!(goals.error.is_some());
        assert!(database.list_goals().unwrap().is_empty());
    }

    #[test]
    fn failed_save_preserves_input() {
        let database = Database::open(":memory:").unwrap();
        let path =
            std::env::temp_dir().join(format!("life-terminal-readonly-{}.db", std::process::id()));
        let setup = rusqlite::Connection::open(&path).unwrap();
        setup.execute_batch("CREATE TABLE goals (id INTEGER PRIMARY KEY, title TEXT NOT NULL, completed INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP); CREATE TRIGGER reject_goal BEFORE INSERT ON goals BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
        let failing_database = Database::open(path.to_str().unwrap()).unwrap();
        let mut goals = Goals::default();
        press(&mut goals, &database, KeyCode::Char('a'));
        press(&mut goals, &database, KeyCode::Char('x'));
        press(&mut goals, &failing_database, KeyCode::Enter);
        assert_eq!(goals.textarea.as_ref().unwrap().lines(), &["x"]);
        assert!(goals.error.as_ref().unwrap().contains("test failure"));
        drop(failing_database);
        drop(setup);
        std::fs::remove_file(path).unwrap();
    }
}

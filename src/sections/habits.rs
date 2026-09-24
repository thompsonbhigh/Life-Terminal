use crate::db::Database;
use crate::widgets::Popup;

use chrono::NaiveDate;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use ratatui_textarea::TextArea;

use super::Section;

/* 
 2. Two-line rows — best for a smaller list

  > ✓ Read 20 minutes
      12-day streak · Completed today

    ○ Exercise
      4-day streak · Last done yesterday
*/

#[derive(Default)]
pub struct Habits {
    list_state: ListState,
    textarea: Option<TextArea<'static>>,
    error: Option<String>,
    popup: Option<Popup<'static>>,
}

impl Habits {
    fn sync_selection(&mut self, habit_count: usize) {
        self.list_state.select(if habit_count == 0 {
            None
        } else {
            Some(self.list_state.selected().unwrap_or(0).min(habit_count - 1))
        });
    }
}

impl Section for Habits {
    fn name(&self) -> &'static str {
        "Habits"
    }

    fn captures_input(&self) -> bool {
        self.textarea.is_some() || self.popup.is_some()
    }

    fn handle_event(&mut self, event: &Event, database: &Database) {
        let Event::Key(key) = event else { return };

        if let Some(_popup) = &mut self.popup {
            match key.code {
                KeyCode::Esc => {
                    self.popup = None;
                    self.error = None;
                }
                KeyCode::Char('d') => {
                    if let Ok(habits) = database.list_habits() {
                        if let Some(habit) = self
                            .list_state
                            .selected()
                            .and_then(|index| habits.get(index))
                        {
                            if let Err(error) = database.delete_habit(habit.id) {
                                self.error = Some(format!("Could not delete habit: {error}"));
                            }
                        }
                    }
                    self.popup = None;
                    self.error = None;
                }
                _ => {

                }
            }
        }

        else if let Some(textarea) = &mut self.textarea {
            match key.code {
                KeyCode::Esc => {
                    self.textarea = None;
                    self.error = None;
                }
                KeyCode::Enter => {
                    let title = textarea.lines().join(" ");
                    let title = title.trim();
                    if title.is_empty() {
                        self.error = Some("Please enter a habit title.".into());
                    } else {
                        match database.add_habit(title) {
                            Ok(()) => {
                                self.textarea = None;
                                self.error = None;
                            }
                            Err(error) => {
                                self.error = Some(format!("Could not save habit: {error}"))
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
            if let Ok(habits) = database.list_habits() {
                self.sync_selection(habits.len());
                if let Some(selected) = self.list_state.selected() {
                    let next = match key.code {
                        KeyCode::Char('j') | KeyCode::Down => (selected + 1).min(habits.len() - 1),
                        _ => selected.saturating_sub(1),
                    };
                    self.list_state.select(Some(next));
                }
            }
        } else if key.code == KeyCode::Char('a') {
            let mut textarea = TextArea::default();
            textarea.set_block(
                Block::default()
                    .title("Add a habit")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );
            textarea.set_placeholder_text("Enter a habit");
            textarea.set_cursor_line_style(Style::default());
            self.textarea = Some(textarea);
            self.error = None;
        } else if key.code == KeyCode::Char(' ') {
            if let Ok(habits) = database.list_habits() {
                if let Some(habit) = self
                    .list_state
                    .selected()
                    .and_then(|index| habits.get(index))
                {
                    if let Err(error) = database.toggle_habit(habit.id) {
                        self.error = Some(format!("Could not update habit: {error}"));
                    }
                }
            }
        } else if key.code == KeyCode::Char('d') {
            let popup = Popup::default()
                .content("Are you sure you want to delete this habit? (d/Esc)")
                .style(Style::new().red())
                .title("Confirm Delete")
                .title_style(Style::new().red().bold())
                .border_style(Style::new().red());
            self.popup = Some(popup);
            self.error = None;
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        let habits = database.list_habits();

        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(26)])
            .split(area);

        let block = Block::bordered()
            .title("Habits")
            .padding(Padding::proportional(1));
        match habits {
            Ok(habits) if !habits.is_empty() => {
                self.sync_selection(habits.len());
                let items = habits.iter().map(|habit| {
                    let mark = if habit.completed { "✓" } else { "○" };
                    let last_complete_date = if habit.last_completed == "NEVER" {
                         "never".to_string()
                    } else { 
                        let date = NaiveDate::parse_from_str(habit.last_completed.as_str(), "%Y-%m-%d");
                        match date {
                            Ok(date) => date.format("%b %d").to_string(),
                            Err(_) => habit.last_completed.clone(),
                        }
                    };
                    ListItem::new(vec![
                        Line::from(format!("{mark} {}", habit.title)),
                        Line::styled(
                            format!("  {}-day streak · Last completed {last_complete_date}", habit.streak),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ])
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
                    Ok(_) => "No habits yet - press [a] to add one.".to_string(),
                    Err(error) => format!("Could not load habits: {error}"),
                };
                frame.render_widget(Paragraph::new(message).block(block), panes[0]);
            }
        }

        frame.render_widget(
            Paragraph::new(
                "[a] Add habit\n\
               \n\
                [d] Delete habit\n\
               \n\
                [Space] Complete\n\
               \n\
                [j/↓] Next habit\n\
                \n\
                [k/↑] Previous habit",
            )
            .block(
                Block::bordered()
                    .title("Habit Controls")
                    .padding(Padding::proportional(1)),
            ),
            panes[1],
        );

        if let Some(popup) = &self.popup {
            let popup_area = panes[0];
            let width = popup_area.width.min(60);
            let height = popup_area.height.min(7);
            let popup_rect = Rect::new(
                popup_area.x + (popup_area.width - width) / 2,
                popup_area.y + (popup_area.height - height) / 2,
                width,
                height,
            );
            let rows = Layout::vertical([Constraint::Min(3), Constraint::Length(2)]).split(popup_rect);
            frame.render_widget(popup, rows[0]);
        }

        if let Some(textarea) = &self.textarea {
            let habit_area = panes[0];
            let width = habit_area.width.min(60);
            let height = habit_area.height.min(7);
            let popup = Rect::new(
                habit_area.x + (habit_area.width - width) / 2,
                habit_area.y + (habit_area.height - height) / 2,
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

    fn press(habits: &mut Habits, database: &Database, code: KeyCode) {
        habits.handle_event(
            &Event::Key(KeyEvent::new(code, KeyModifiers::NONE)),
            database,
        );
    }

    #[test]
    fn empty_title_is_rejected() {
        let database = Database::open(":memory:").unwrap();
        let mut habits = Habits::default();
        for code in [KeyCode::Char('a'), KeyCode::Char(' '), KeyCode::Enter] {
            press(&mut habits, &database, code);
        }
        assert!(habits.captures_input());
        assert!(habits.error.is_some());
        assert!(database.list_habits().unwrap().is_empty());
    }

    #[test]
    fn failed_save_preserves_input() {
        let database = Database::open(":memory:").unwrap();
        let path =
            std::env::temp_dir().join(format!("life-terminal-readonly-{}.db", std::process::id()));
        let setup = rusqlite::Connection::open(&path).unwrap();
        setup.execute_batch("CREATE TABLE habits (id INTEGER PRIMARY KEY, title TEXT NOT NULL, completed INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP); CREATE TRIGGER reject_habit BEFORE INSERT ON habits BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
        let failing_database = Database::open(path.to_str().unwrap()).unwrap();
        let mut habits = Habits::default();
        press(&mut habits, &database, KeyCode::Char('a'));
        press(&mut habits, &database, KeyCode::Char('x'));
        press(&mut habits, &failing_database, KeyCode::Enter);
        assert_eq!(habits.textarea.as_ref().unwrap().lines(), &["x"]);
        assert!(habits.error.as_ref().unwrap().contains("test failure"));
        drop(failing_database);
        drop(setup);
        std::fs::remove_file(path).unwrap();
    }
}

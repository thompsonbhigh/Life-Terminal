use crate::db::Database;
use crate::widgets::Popup;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use ratatui_textarea::TextArea;

use super::Section;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum TextAreaType {
    #[default]
    Goal,
    SubTask,
}

#[derive(Default)]
pub struct Goals {
    list_state: ListState,
    list_state_subtask: ListState,
    subtask_active: bool,
    textarea: Option<TextArea<'static>>,
    textarea_type: TextAreaType,
    current_goal_id: i64,
    error: Option<String>,
    popup: Option<Popup<'static>>,
}

impl Goals {
    fn sync_selection(&mut self, goal_count: usize) {
        self.list_state.select(if goal_count == 0 {
            None
        } else {
            Some(self.list_state.selected().unwrap_or(0).min(goal_count - 1))
        });
    }

    fn sync_selection_subtask(&mut self, subtask_count: usize) {
        self.list_state_subtask.select(if subtask_count == 0 {
            None
        } else {
            Some(self.list_state_subtask.selected().unwrap_or(0).min(subtask_count - 1))
        });
    }
}

impl Section for Goals {
    fn name(&self) -> &'static str {
        "Goals"
    }

    fn captures_input(&self) -> bool {
        self.textarea.is_some() || self.popup.is_some()
    }

    fn handle_event(&mut self, event: &Event, database: &Database) {
        let Event::Key(key) = event else { return };

        if self.popup.is_some() {
            match key.code {
                KeyCode::Esc => {
                    self.popup = None;
                    self.error = None;
                }
                KeyCode::Char('d') => {
                    self.error = None;
                    if !self.subtask_active {
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
                    } else {
                        if let Ok(subtasks) = database.list_subtasks(self.current_goal_id) {
                            if let Some(subtask) = self
                                .list_state_subtask
                                .selected()
                                .and_then(|index| subtasks.get(index))
                            {
                                if let Err(error) = database.delete_subtask(subtask.id, self.current_goal_id) {
                                    self.error = Some(format!("Could not delete subtask: {error}"));
                                }
                            }
                        }
                    }
                    self.popup = None;
                }
                _ => {}
            }
        } else if let Some(textarea) = &mut self.textarea {
            match key.code {
                KeyCode::Esc => {
                    self.textarea = None;
                    self.error = None;
                }
                KeyCode::Enter => {
                    let title = textarea.lines().join(" ");
                    let title = title.trim();

                    if self.textarea_type == TextAreaType::Goal {
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
                    } else if self.textarea_type == TextAreaType::SubTask {
                        if title.is_empty() {
                            self.error = Some("Please enter a subtask title.".into());
                        } else {
                            match database.add_subtask(self.current_goal_id, title) {
                                Ok(()) => {
                                    self.textarea = None;
                                    self.error = None;
                                }
                                Err(error) => {
                                    self.error = Some(format!("Could not save subtask: {error}"))
                                }
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
            if !self.subtask_active {
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
            } else {
                if let Ok(subtasks) = database.list_subtasks(self.current_goal_id) {
                    self.sync_selection(subtasks.len());
                    if let Some(selected) = self.list_state_subtask.selected() {
                        let next = match key.code {
                            KeyCode::Char('j') | KeyCode::Down => (selected + 1).min(subtasks.len() - 1),
                            _ => selected.saturating_sub(1),
                        };
                        self.list_state_subtask.select(Some(next));
                    }
                }
            }
        } else if key.code == KeyCode::Char('a') {
            let mut textarea = TextArea::default();
            textarea.set_block(
                Block::default()
                    .title("Add a goal")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );
            textarea.set_placeholder_text("Enter a goal");
            textarea.set_cursor_line_style(Style::default());
            self.textarea = Some(textarea);
            self.textarea_type = TextAreaType::Goal;
            self.error = None;
        } else if key.code == KeyCode::Char('A') {
            if let Ok(goals) = database.list_goals() {
                if let Some(goal) = self
                    .list_state
                    .selected()
                    .and_then(|index| goals.get(index))
                {
                    self.current_goal_id = goal.id;
                }
            }

            let mut textarea = TextArea::default();
            textarea.set_block(
                Block::default()
                    .title("Add a subtask")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );
            textarea.set_placeholder_text("Enter a subtask");
            textarea.set_cursor_line_style(Style::default());
            self.textarea = Some(textarea);
            self.textarea_type = TextAreaType::SubTask;
            self.error = None;
        } else if key.code == KeyCode::Char(' ') {
            if !self.subtask_active {
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
            } else {
                if let Ok(subtasks) = database.list_subtasks(self.current_goal_id) {
                    if let Some(subtask) = self
                        .list_state_subtask
                        .selected()
                        .and_then(|index| subtasks.get(index))
                    {
                        if let Err(error) = database.toggle_subtask(subtask.id, self.current_goal_id) {
                            self.error = Some(format!("Could not update subtask: {error}"));
                        }
                    }
                }
            }
        } else if key.code == KeyCode::Char('d') {
            let popup = Popup::default()
                .content(if self.subtask_active { "Are you sure you want to delete this subtask? (d/Esc)" } else { "Are you sure you want to delete this goal? (d/Esc)" })
                .style(Style::new().red())
                .title("Confirm Delete")
                .title_style(Style::new().red().bold())
                .border_style(Style::new().red());
            self.popup = Some(popup);
            self.error = None;
        } else if key.code == KeyCode::Enter && (database.list_subtasks(self.current_goal_id).as_ref().map_or(0, |subtasks| subtasks.len())) > 0 {
            self.subtask_active = true;
        } else if key.code == KeyCode::Esc && self.subtask_active {
            self.subtask_active = false;
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        let goals = database.list_goals();
        self.sync_selection(goals.as_ref().map_or(0, |goals| goals.len()));
        let selected_goal = goals.as_ref().ok().and_then(|goals| {
            self.list_state
                .selected()
                .and_then(|index| goals.get(index))
        });
        self.current_goal_id = selected_goal.map_or(0, |goal| goal.id);

        let subtasks = database.list_subtasks(self.current_goal_id);
        self.sync_selection_subtask(subtasks.as_ref().map_or(0, |subtasks| subtasks.len()));
        let _selected_subtask = subtasks.as_ref().ok().and_then(|subtasks| {
            self.list_state_subtask
                .selected()
                .and_then(|index| subtasks.get(index))
        });

        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(26)])
            .split(area);

        let inner_panes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(panes[0]);

        let block = Block::bordered()
            .title("Goals")
            .padding(Padding::proportional(1));
        match &goals {
            Ok(goals) if !goals.is_empty() => {
                self.sync_selection(goals.len());
                let items = goals.iter().map(|goal| {
                    let mark = if goal.completed { "✓" } else { "○" };

                    let ratio = if goal.subtask_count > 0 {
                        goal.progress as f64 / goal.subtask_count as f64
                    } else if goal.completed {
                        1.0
                    } else {
                        0.0
                    }
                    .clamp(0.0, 1.0);

                    let width = 20;
                    let filled = (ratio * width as f64).round() as usize;
                    let bar = format!(
                        "{}{}",
                        "█".repeat(filled),
                        "░".repeat(width - filled),
                    );

                    ListItem::new(format!(
                        "{mark} {}\n  {bar} {:.0}%",
                        goal.title,
                        ratio * 100.0,
                    ))
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
                    inner_panes[0],
                    &mut self.list_state,
                );
            }
            result => {
                self.list_state.select(None);
                let message = match result {
                    Ok(_) => "No goals yet - press [a] to add one.".to_string(),
                    Err(error) => format!("Could not load goals: {error}"),
                };
                frame.render_widget(Paragraph::new(message).block(block), inner_panes[0]);
            }
        }

        let subtask_block = Block::bordered()
            .title("Subtasks")
            .padding(Padding::proportional(1));
        match subtasks {
            Ok(subtasks) if !subtasks.is_empty() => {
                let items = subtasks.iter().map(|subtask| {
                    let mark = if subtask.completed { "✓" } else { "○" };
                    ListItem::new(format!("{mark} {}", subtask.title))
                });
                frame.render_stateful_widget(
                    List::new(items)
                        .block(subtask_block)
                        .highlight_style(
                            Style::default()
                                .bg(if self.subtask_active { Color::Cyan } else { Color::Reset })
                                .fg(if self.subtask_active { Color::Black } else { Color::Reset })
                                .add_modifier(if self.subtask_active { Modifier::BOLD } else { Modifier::empty() }),
                        )
                        .highlight_symbol(if self.subtask_active { "> " } else { "  " }),
                    inner_panes[1],
                    &mut self.list_state_subtask,
                );
            }
            result => {
                let message = match result {
                    Ok(_) if selected_goal.is_none() => {
                        "Select a goal to view subtasks.".to_string()
                    }
                    Ok(_) => "No subtasks yet - press [A] to add one.".to_string(),
                    Err(error) => format!("Could not load subtasks: {error}"),
                };
                frame.render_widget(Paragraph::new(message).block(subtask_block), inner_panes[1]);
            }
        }

        frame.render_widget(
            Paragraph::new(
                "[a] Add goal\n\
               \n\
                [A] Add subtask\n\
               \n\
                [d] Delete goal\n\
               \n\
                [Space] Complete\n\
               \n\
                [Enter] View subtasks\n\
               \n\
                [j/↓] Next goal\n\
                \n\
                [k/↑] Previous goal",

            )
            .block(
                Block::bordered()
                    .title("Goal Controls")
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
    fn nested_panes_keep_goals_visible_and_subtasks_follow_selection() {
        let database = Database::open(":memory:").unwrap();
        database.add_goal("First goal").unwrap();
        let first = database.list_goals().unwrap()[0].id;
        database.add_subtask(first, "First child").unwrap();
        database.add_goal("Second goal").unwrap();
        let mut goals = Goals::default();
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 24)).unwrap();
        for (selection, child_visible) in [(0, false), (1, true)] {
            goals.list_state.select(Some(selection));
            terminal
                .draw(|frame| goals.render(frame, frame.area(), &database))
                .unwrap();
            let buffer = terminal.backend().buffer();
            let rows: Vec<String> = (0..24)
                .map(|y| (0..100).map(|x| buffer[(x, y)].symbol()).collect())
                .collect();
            assert!(rows.iter().any(|row| row.contains("First goal")));
            assert!(rows.iter().any(|row| row.contains("Second goal")));
            assert_eq!(
                rows.iter().any(|row| row.contains("First child")),
                child_visible
            );
            assert_eq!(goals.list_state.selected(), Some(selection));
            let subtask_row = rows
                .iter()
                .position(|row| row.contains("Subtasks"))
                .unwrap();
            assert!(rows[subtask_row - 1].chars().take(74).all(|ch| ch == ' '));
            assert!((0..24).all(|y| buffer[(74, y)].symbol() == " "));
        }
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
            std::env::temp_dir().join(format!("life-terminal-goals-readonly-{}.db", std::process::id()));
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

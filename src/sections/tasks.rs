use crossterm::event::{Event, KeyCode, read};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};
use ratatui_textarea::TextArea;
use crate::db::Database;

use super::Section;

pub struct Tasks;

impl Section for Tasks {
    fn name(&self) -> &'static str {
        "Tasks"
    }

    fn handle_event(&mut self, event: &Event, database: &Database) {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('a') {
                if let Err(error) = database.add_task("New task") {
                    eprintln!("Could not add task: {error}");
                }
            }
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
            .constraints([
                Constraint::Min(30),
                Constraint::Length(24),
            ])
            .split(area);

        frame.render_widget(
            Paragraph::new(content)
                .block(Block::bordered().title("Tasks")),
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
            .block(Block::bordered().title("Task Controls")
            .padding(Padding::proportional(1))),
            panes[1],
        );
    }
}

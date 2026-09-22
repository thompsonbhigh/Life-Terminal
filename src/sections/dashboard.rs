use crossterm::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use crate::db::Database;

use super::Section;

pub struct Dashboard;

impl Section for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn handle_event(&mut self, _: &Event, database: &Database) {}

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .spacing(3)
            .split(area);
        
        let content = match database.list_tasks() {
            Ok(tasks) if tasks.is_empty() => "No tasks yet".to_string(),
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

        let tasks = Paragraph::new(content)
            .block(
                Block::default()
                    .title(" Tasks ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: true });

        let goals = Paragraph::new("Rust TUI\n██████░░░░ 60%\n\nRun a 5K\n███░░░░░░░ 30%")
            .block(
                Block::default()
                    .title(" Goals ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: true });

        let habits = Paragraph::new("✓ Exercise\n✓ Read\n○ Meditate\n○ Journal")
            .block(
                Block::default()
                    .title(" Habits ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(tasks, columns[0]);
        frame.render_widget(goals, columns[1]);
        frame.render_widget(habits, columns[2]);
    }
}

use crossterm::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    style::{Style, Color},
    text::Line,
    Frame,
};
use chrono::NaiveDate;
use crate::db::Database;

use super::Section;

pub struct Dashboard;

impl Section for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn handle_event(&mut self, _: &Event, _database: &Database) {}

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
        
        let task_content = match database.list_tasks() {
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

        let tasks = Paragraph::new(task_content)
            .block(
                Block::default()
                    .title(" Tasks ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: true });

        let goal_content = match database.list_goals() {
            Ok(goals) if goals.is_empty() => vec![Line::from("No goals yet")],
            Ok(goals) => goals
                .iter().flat_map(|goal| {
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
                vec![
                    Line::from(format!("{mark} {}", goal.title)),
                    Line::styled(
                        format!("  {bar} {:.0}%", ratio * 100.0),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Line::default(),
                ]
                })
                .collect::<Vec<_>>(),
            Err(error) => vec![Line::from(format!("Could not load goals: {error}"))],
        };

        let goals = Paragraph::new(goal_content)
            .block(
                Block::default()
                    .title(" Goals ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: false });

        let habit_content = match database.list_habits() {
            Ok(habits) if habits.is_empty() => vec![Line::from("No habits yet")],
            Ok(habits) => habits
                .iter().flat_map(|habit| {
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
                    vec![
                        Line::from(format!("{mark} {}", habit.title)),
                        Line::styled(
                            format!("  {}-day streak · Last completed {last_complete_date}", habit.streak),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Line::default(),
                    ]
                })
                .collect::<Vec<_>>(),
            Err(error) => vec![Line::from(format!("Could not load habits: {error}"))],
        };

        let habits = Paragraph::new(habit_content)
            .block(
                Block::default()
                    .title(" Habits ")
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(tasks, columns[0]);
        frame.render_widget(goals, columns[1]);
        frame.render_widget(habits, columns[2]);
    }
}

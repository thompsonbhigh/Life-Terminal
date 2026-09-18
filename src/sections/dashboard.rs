use crossterm::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap, Padding},
    Frame,
};

use super::Section;

pub struct Dashboard;

impl Section for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn handle_event(&mut self, _: &Event) {}

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .spacing(3)
            .split(area);

        let tasks = Paragraph::new("• Review project\n• Buy groceries\n• Reply to email")
          .block(Block::default()
            .title(" Tasks ")
            .borders(Borders::ALL)
            .padding(Padding::proportional(1))
        )
          .wrap(Wrap { trim: true });

        let goals = Paragraph::new("Rust TUI\n██████░░░░ 60%\n\nRun a 5K\n███░░░░░░░ 30%")
            .block(Block::default()
                .title(" Goals ")
                .borders(Borders::ALL)
                .padding(Padding::proportional(1))
            )
            .wrap(Wrap { trim: true });

        let habits = Paragraph::new("✓ Exercise\n✓ Read\n○ Meditate\n○ Journal")
            .block(Block::default()
                .title(" Habits ")
                .borders(Borders::ALL)
                .padding(Padding::proportional(1))
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(tasks, columns[0]);
        frame.render_widget(goals, columns[1]);
        frame.render_widget(habits, columns[2]);
    }
}

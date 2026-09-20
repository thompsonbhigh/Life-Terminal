use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::db::Database;

use super::Section;

pub struct Habits;

impl Section for Habits {
    fn name(&self) -> &'static str {
        "Habits"
    }

    fn handle_event(&mut self, _: &Event, database: &Database) {}

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database) {
        frame.render_widget(
            Paragraph::new("Habits\n\nSection coming soon")
                .block(Block::default().borders(Borders::ALL)),
            area,
        );
    }
}

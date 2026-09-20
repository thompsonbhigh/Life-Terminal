mod dashboard;
mod goals;
mod habits;
mod notes;
mod study;
mod tasks;

use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};
use crate::db::Database;

pub trait Section {
    fn name(&self) -> &'static str;
    fn handle_event(&mut self, event: &Event, database: &Database);
    fn update(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database);
}

pub fn default_sections() -> Vec<Box<dyn Section>> {
    vec![
        Box::new(dashboard::Dashboard),
        Box::new(tasks::Tasks),
        Box::new(goals::Goals),
        Box::new(habits::Habits),
        Box::new(study::Study),
        Box::new(notes::Notes),
    ]
}

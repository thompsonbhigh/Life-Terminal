mod dashboard;
mod goals;
mod habits;
mod notes;
mod study;
mod tasks;

use crate::db::Database;
use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub trait Section {
    fn name(&self) -> &'static str;
    fn captures_input(&self) -> bool {
        false
    }
    fn handle_event(&mut self, event: &Event, database: &Database);
    fn update(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect, database: &Database);
}

pub fn default_sections() -> Vec<Box<dyn Section>> {
    vec![
        Box::new(dashboard::Dashboard),
        Box::new(tasks::Tasks::default()),
        Box::new(goals::Goals::default()),
        Box::new(habits::Habits::default()),
        Box::new(study::Study),
        Box::new(notes::Notes),
    ]
}

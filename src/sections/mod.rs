mod dashboard;
mod goals;
mod habits;
mod notes;
mod study;
mod tasks;

use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub trait Section {
    fn name(&self) -> &'static str;
    fn handle_event(&mut self, event: &Event);
    fn update(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect);
}

pub fn default_sections() -> Vec<Box<dyn Section>> {
    vec![
        Box::new(dashboard::Dashboard),
        Box::new(goals::Goals),
        Box::new(study::Study),
        Box::new(tasks::Tasks),
        Box::new(habits::Habits),
        Box::new(notes::Notes),
    ]
}

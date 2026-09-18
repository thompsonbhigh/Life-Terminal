use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::Section;

pub struct Study;

impl Section for Study {
    fn name(&self) -> &'static str {
        "Study"
    }

    fn handle_event(&mut self, _: &Event) {}

    fn update(&mut self) {}

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Study\n\nSection coming soon")
                .block(Block::default().borders(Borders::ALL)),
            area,
        );
    }
}

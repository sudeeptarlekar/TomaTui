use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Debug, Default)]
pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub status: Status,
}

#[derive(Debug, Default)]
pub enum Priority {
    High,
    #[default]
    Medium,
    Low,
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Pending,
    Finished,
}

impl Widget for Task {
    fn render(self, _area: Rect, _buffer: &mut Buffer) {}
}

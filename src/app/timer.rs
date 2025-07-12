use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Default, Debug)]
pub struct Timer {
    pub time: usize,
    pub state: State,
    pub pomodoro_style: Pomodoro,
}

#[derive(Default, Debug)]
pub enum State {
    #[default]
    Paused,
    Running,
}

#[derive(Debug, Default)]
pub enum Pomodoro {
    #[default]
    Short,
    Long,
}

impl Widget for Timer {
    fn render(self, _area: Rect, _buffer: &mut Buffer) {}
}

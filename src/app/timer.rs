use ratatui::prelude::{Buffer, Frame, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::border::THICK;
use ratatui::text::Line;
use ratatui::widgets::{Borders, Paragraph, Widget, block::Block};

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

impl Timer {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area)
    }
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::raw("Timer");
        let instructions = Line::from(vec![
            "[Start/Stop ".into(),
            "s] ".bold().blue(),
            "[Break ".into(),
            "b] ".bold().blue(),
            "[Short/Long ".into(),
            "l]".bold().blue(),
        ]);

        let block = Block::new()
            .borders(Borders::all())
            .border_set(THICK)
            .title(title.left_aligned())
            .title_bottom(instructions.centered());

        Paragraph::new("Coming Soon!")
            .centered()
            .block(block)
            .render(area, buf);
    }
}

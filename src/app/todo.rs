use ratatui::prelude::{Buffer, Frame, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, block::Block};

#[derive(Default, Debug)]
pub struct ToDo {
    pub tasks: Vec<Task>,
}

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

impl ToDo {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &ToDo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::raw("Tasks");

        let instructions = Line::from(vec![
            "[Create ".into(),
            "a".bold().green(),
            "] [Delete ".into(),
            "d".bold().red(),
            "] [Mark ".into(),
            "m".bold().blue(),
            "]".into(),
        ]);

        let block = Block::bordered()
            .border_set(ratatui::symbols::border::THICK)
            .title(title.centered())
            .title_bottom(instructions.centered());

        Paragraph::new("Comming Soon!")
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

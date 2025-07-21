use ratatui::prelude::{Buffer, Frame, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Borders, Paragraph, Widget, block::Block};

#[derive(Debug, Default)]
pub struct Player {
    pub url: String,
    pub streaming_online: bool,
    pub state: PlayerState,
}

#[derive(Default, Debug)]
pub enum PlayerState {
    Playing,
    #[default]
    Paused,
}

impl Player {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &Player {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::raw("Player");
        let instructions = Line::from(vec![
            "[Play/Pause ".into(),
            "Space".blue().bold(),
            "] [Next ".into(),
            "n".blue().bold(),
            "] [Previous ".into(),
            "p".blue().bold(),
            "]".into(),
        ]);
        let block = Block::new()
            .borders(Borders::all())
            .border_set(ratatui::symbols::border::THICK)
            .title(title.left_aligned())
            .title_bottom(instructions.centered());

        Paragraph::new("todo!")
            .centered()
            .block(block)
            .render(area, buf);
    }
}

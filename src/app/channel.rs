use ratatui::{
    prelude::{Buffer, Frame, Rect},
    style::Stylize,
    symbols::{block, border::THICK},
    text::Line,
    widgets::{Borders, Paragraph, Widget, block::Block},
};

#[derive(Debug, Default)]
pub struct ChannelList {
    pub list: Channel,
}

#[derive(Debug, Default)]
pub struct Channel {
    pub name: String,
    pub url: String,
    pub country_code: String,
    bitrate: usize,
    format: String,
}

impl ChannelList {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &ChannelList {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::raw("Channels");
        let instructions = Line::from(vec![
            "[Next ".into(),
            "c+n".bold().yellow(),
            "] [Previous ".into(),
            "c+p".bold().yellow(),
            "] [Select ".into(),
            "c+#".bold().yellow(),
            "]".into(),
        ]);

        let block = Block::bordered()
            .border_set(THICK)
            .title(title.left_aligned())
            .title_bottom(instructions.centered());
        Paragraph::new("Coming soon")
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

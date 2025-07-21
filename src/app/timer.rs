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
            "s".bold().blue(),
            "] [Break ".into(),
            "b".bold().blue(),
            "] [Short/Long ".into(),
            "l".bold().blue(),
            "]".into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::owo_colors::OwoColorize;
    use ratatui::style::Style;

    #[test]
    fn draw() {
        let timer = Timer::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        timer.render(buf.area, &mut buf);

        let key_style = Style::new().bold().blue();

        let mut expected = Buffer::with_lines(vec![
            "┏Timer━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃                  Coming Soon!                  ┃",
            "┃                                                ┃",
            "┗━━━━[Start/Stop s] [Break b] [Short/Long l]━━━━━┛",
        ]);

        expected.set_style(Rect::new(17, 3, 1, 4), key_style);
        expected.set_style(Rect::new(27, 3, 1, 4), key_style);
        expected.set_style(Rect::new(42, 3, 1, 4), key_style);

        assert_eq!(buf, expected);
    }
}

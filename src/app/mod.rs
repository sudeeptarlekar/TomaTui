pub mod player;
pub mod timer;
pub mod todo;

use std::thread;

use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};

use player::Player;
use timer::Timer;
use todo::ToDo;

#[derive(Debug, Default)]
pub struct App {
    // Is app running?
    pub running: bool,
    pub todo_list: ToDo,
    pub timer: Timer,
    pub player: Player,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        let mut i = 0;
        while self.running && i < 1 {
            terminal.draw(|frame| self.draw(frame))?;
            // event::handle_event()?;
            thread::sleep(std::time::Duration::from_secs(10));
            i += 1;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let outer_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(frame.area());

        let inner_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ])
            .split(outer_layouts[0]);

        self.player.draw(frame, inner_layouts[0]);
        self.timer.draw(frame, inner_layouts[1]);
        self.todo_list.draw(frame, outer_layouts[1]);
        frame.render_widget(
            Paragraph::new("Channel List")
                .centered()
                .block(Block::new().borders(Borders::all())),
            inner_layouts[2],
        );
    }
}

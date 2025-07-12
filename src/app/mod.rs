pub mod event;
pub mod task;
pub mod timer;

use ratatui::DefaultTerminal;
use ratatui::Frame;

use task::Task;
use timer::Timer;

#[derive(Debug, Default)]
pub struct App {
    // Is app running?
    pub running: bool,
    pub task_list: Vec<Task>,
    pub timer: Timer,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            event::handle_event()?;
        }
        Ok(())
    }

    fn draw(&self, _frame: &mut Frame) {
        todo!();
    }
}

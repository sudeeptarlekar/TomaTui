pub mod app;

fn main() -> color_eyre::Result<()> {
    let mut app = app::App::new();
    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

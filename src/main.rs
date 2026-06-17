mod app;
mod tests;

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};



use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}



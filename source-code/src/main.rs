mod app;
mod config;
mod editor;
mod highlight;
mod ui;
mod filetree;
mod terminal_panel;
mod marketplace;
mod keybinds;
mod languages;
mod welcome;
mod plugins;
mod utils;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|s| s.as_str());
    let mut app = App::new(path)?;
    app.run()?;
    Ok(())
}

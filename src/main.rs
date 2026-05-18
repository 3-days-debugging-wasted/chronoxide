use ratatui;
use anyhow::Result;
mod tui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = tui::run_app(&mut terminal);
    ratatui::restore();
    result
}

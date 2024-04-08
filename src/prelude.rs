pub use anyhow::Result;

pub use crate::screen_alt::AltScreen;
pub use crate::screen_main::MainScreen;
pub use crate::draw_cmd::DrawCmd;

pub use crossterm::cursor as xCursor;
pub use crossterm::event as xEvent;
pub use crossterm::style::Color as xColor;
pub use crossterm::style::Colors as xColors;
pub use crossterm::terminal as xTerm;

pub fn set_title(title: &str) -> Result<()> {
    use crossterm::ExecutableCommand;
    use std::io::stdout;
    stdout().execute(crossterm::terminal::SetTitle(title))?;
    Ok(())
}

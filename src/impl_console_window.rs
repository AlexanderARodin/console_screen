use crate::prelude::*;
use anyhow::anyhow;

//  //  //  //  //  //  //  //  //  //
use crossterm::cursor as xCursor;
use crossterm::style as xStyle;
use crossterm::terminal as xTerm;
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(0);
//  //  //  //  //  //  //  //  //  //
//          IMPL
//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub fn capture_mouse() -> Result<()> {
        stdout().execute(xEvent::EnableMouseCapture)?;
        Ok(())
    }
    pub fn get_painter(&self) -> Result<ConsoleDraw> {
        match self.state {
            ConsoleWindowState::Alt(_) => ConsoleDraw::new(),
            ConsoleWindowState::NotTerminal => Err(anyhow!("Can't paint. It's not a terminal")),
            ConsoleWindowState::Main => Err(anyhow!("Painting is not implemeted in Main screen")),
        }
    }
    pub fn read_events() -> Result<Vec<xEvent::Event>> {
        let mut result = Vec::new();
        while xEvent::poll(POLL_WAIT_TIME)? {
            result.push(xEvent::read()?);
        }
        Ok(result)
    }

    pub fn info(&mut self, info: &str) {
        let _ = self.println_on_main_screen(false, info);
    }
    pub fn error(&mut self, msg: &str) {
        let _ = self.println_on_main_screen(true, msg);
    }
    fn println_on_main_screen(&mut self, is_error: bool, line: &str) -> Result<()> {
        let automouse_capture = match self.state {
            ConsoleWindowState::Alt(au) => Some(au),
            _ => None,
        };
        let res = self.restore_main_screen();
        {
            if is_error {
                eprintln!("{line}");
            } else {
                println!("{line}");
            }
        }
        if let Err(e) = res {
            eprintln!("{}", e);
        }
        if let Some(au) = automouse_capture {
            self.enter_alt_screen(au)?;
        }
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub fn restore_main_screen(&mut self) -> Result<()> {
        if let ConsoleWindowState::NotTerminal = self.state {
            return Ok(());
        }
        //
        let mut error_list = String::new();
        //
        if let Err(e) = Self::sync_and_flush() {
            collect_errors(&mut error_list, e.as_ref());
        }
        //
        if let Err(e) = xTerm::disable_raw_mode() {
            collect_errors(&mut error_list, &e);
        }
        let mut stdout = stdout();
        if let Err(e) = stdout.execute(xEvent::DisableMouseCapture) {
            collect_errors(&mut error_list, &e);
        }
        if let Err(e) = stdout.execute(xTerm::LeaveAlternateScreen) {
            collect_errors(&mut error_list, &e);
        }
        if let Err(e) = stdout.execute(xTerm::EnableLineWrap) {
            collect_errors(&mut error_list, &e);
        }
        if let Err(e) = stdout.execute(xCursor::RestorePosition) {
            collect_errors(&mut error_list, &e);
        }
        if let Err(e) = stdout.execute(xCursor::Show) {
            collect_errors(&mut error_list, &e);
        }
        if let Err(e) = stdout.flush() {
            collect_errors(&mut error_list, &e);
        }
        if error_list.is_empty() {
            self.state = ConsoleWindowState::Main;
            Self::sync_and_flush()?;
            Ok(())
        } else {
            Err(anyhow!(error_list))
        }
    }
    pub fn enter_alt_screen(&mut self, automouse_capture: bool) -> Result<()> {
        if let ConsoleWindowState::NotTerminal = self.state {
            return Err(anyhow!("Can't enter AltScreen, It's not a terminal"));
        }
        //
        xTerm::enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.queue(xCursor::SavePosition)?;
        stdout.queue(xTerm::EnterAlternateScreen)?;
        stdout.queue(xTerm::DisableLineWrap)?;
        stdout.queue(xCursor::Hide)?;
        Self::sync_and_flush()?;
        stdout.execute(xTerm::BeginSynchronizedUpdate)?;
        if automouse_capture {
            Self::capture_mouse()?;
        }
        self.state = ConsoleWindowState::Alt(automouse_capture);
        Ok(())
    }
    pub(crate) fn start_sync_frame() -> Result<()> {
        stdout().execute(xTerm::BeginSynchronizedUpdate)?;
        Ok(())
    }
    pub(crate) fn sync_and_flush() -> Result<()> {
        let mut stdout = stdout();
        stdout.flush()?;
        stdout.execute(xTerm::EndSynchronizedUpdate)?;
        Ok(())
    }

    pub(crate) fn clear_screen() -> Result<()> {
        stdout().queue(xTerm::Clear(xTerm::ClearType::All))?;
        Ok(())
    }
    pub fn set_title(title: &str) -> Result<()> {
        stdout().execute(xTerm::SetTitle(title))?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn size() -> Result<(u16, u16)> {
        Ok(xTerm::size()?)
    }
    pub(crate) fn move_to(x: u16, y: u16) -> Result<()> {
        stdout().queue(xCursor::MoveTo(x, y))?;
        Ok(())
    }
    pub(crate) fn print(txt: &str) -> Result<()> {
        stdout().queue(xStyle::Print(txt))?;
        Ok(())
    }
    pub(crate) fn set_colors(colors: xColors) -> Result<()> {
        stdout().queue(xStyle::SetColors(colors))?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
fn collect_errors(error_list: &mut String, err: &dyn std::error::Error) {
    *error_list += "E: ";
    *error_list += &err.to_string();
    *error_list += "\n";
}

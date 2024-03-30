use crossterm::cursor as xCursor;
use crossterm::event as xEvent;
//use crossterm::style as xStyle;
use crossterm::terminal as xTerm;
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

//  //  //  //  //  //  //  //  //  //
use anyhow::anyhow;
use anyhow::Result;

//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
use crate::screen_drawer::*;
use crate::screen_main::*;

pub struct AltScreen {
    auto_mouse: bool,
}

impl AltScreen {
    pub fn new(auto_mouse: bool) -> Result<Self> {
        p("<-- AltScreen::new");
        //
        try_to_enter_altscreen(auto_mouse)?;
        Ok(Self { auto_mouse })
    }
    pub fn go_main_screen(self) -> Result<MainScreen> {
        drop(self);
        try_to_leave_altscreen()?;
        MainScreen::new()
    }
    //pub fn paint(&self, commands: &[impl crossterm::Command]) -> Result<()> {
    pub fn redraw_all(&self, page: &ScreenDrawer) -> Result<()> {
        let mut stdout = stdout();
        sync_and_flush()?;
        stdout.execute(xTerm::BeginSynchronizedUpdate)?;
        {
            for cmd in commands {
                // TODO
                stdout.queue(cmd);
            }
        }
        sync_and_flush()?;
        Ok(())
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        if let Err(e) = try_to_leave_altscreen() {
            eprintln!("{}", e);
        }
        p("<-- Drop AltScreen");
    }
}

fn try_to_enter_altscreen(auto_mouse: bool) -> Result<()> {
    xTerm::enable_raw_mode()?;
    {
        let mut stdout = stdout();
        stdout.queue(xCursor::SavePosition)?;
        stdout.queue(xTerm::EnterAlternateScreen)?;
        stdout.queue(xTerm::Clear(xTerm::ClearType::All))?;
        stdout.queue(xCursor::Hide)?;
        stdout.queue(xTerm::DisableLineWrap)?;
        sync_and_flush()?;
        stdout.execute(xTerm::BeginSynchronizedUpdate)?;
        if auto_mouse {
            stdout.execute(xEvent::EnableMouseCapture)?;
        }
    }
    Ok(())
}
fn try_to_leave_altscreen() -> Result<()> {
    let mut error_list = String::new();
    {
        if let Err(e) = sync_and_flush() {
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
    }
    if error_list.is_empty() {
        sync_and_flush()?;
        Ok(())
    } else {
        Err(anyhow!(error_list))
    }
}

fn sync_and_flush() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(xTerm::EndSynchronizedUpdate)?;
    //stdout.flush()?;
    Ok(())
}

//  //  //  //  //  //  //  //  //  //
fn collect_errors(error_list: &mut String, err: &dyn std::error::Error) {
    *error_list += "E: ";
    *error_list += &err.to_string();
    *error_list += "\n";
}

//##//##//##//##//##//##//##//##//##//##//##//##//##
fn p(t: &str) {
    std::io::stdout().flush().unwrap();
    println!("# {}", t);
    std::io::stdout().flush().unwrap();
}
//##//##//##//##//##//##//##//##//##//##//##//##//##

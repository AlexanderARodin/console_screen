use crate::prelude::*;

//  //  //  //  //  //  //  //  //  //
use std::io::{stdout,Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal as xTerm;
use crossterm::cursor   as xCursor;
use crossterm::style    as xStyle;

static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(0);
//  //  //  //  //  //  //  //  //  //
//          IMPL
//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub fn capture_mouse() -> Result< () > {
        stdout().execute( xEvent::EnableMouseCapture )?;
        Ok(())
    }
    pub fn get_painter() -> Result< ConsoleDraw > {
        ConsoleDraw::new()
    }
    pub fn read_events() -> Result< Vec<xEvent::Event> > {
        let mut result = Vec::new();
        while xEvent::poll( POLL_WAIT_TIME )? {
            result.push( xEvent::read()? );
        }
        return Ok( result );
    }
    
    pub fn info(info: &str) {
        let _ = Self::println_on_log_screen(false, info);
    }
    pub fn error(msg: &str) {
        let _ = Self::println_on_log_screen(true, msg );
    }
    fn println_on_log_screen(is_error: bool, line: &str ) -> Result< () > {
        Self::restore_log_screen();
        {
            if is_error {
                eprintln!("{line}");
            }else{
                println!("{line}");
            }
        }
        Self::switch_main_screen()?;
        Self::sync_and_flush()?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn restore_log_screen() {
        let _ = xTerm::disable_raw_mode();
        let mut stdout = stdout();
        let _ = stdout.execute( xEvent::DisableMouseCapture );
        let _ = stdout.execute(  xTerm::LeaveAlternateScreen );
        let _ = stdout.queue(    xTerm::EnableLineWrap);
        let _ = stdout.execute(xCursor::RestorePosition );
        let _ = stdout.execute(xCursor::Show );
    }
    pub(crate) fn switch_main_screen() -> Result< () > {
        xTerm::enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute( xTerm::BeginSynchronizedUpdate )?;
        stdout.queue( xCursor::SavePosition )?;
        stdout.queue(   xTerm::EnterAlternateScreen)?;
        stdout.queue(   xTerm::DisableLineWrap)?;
        stdout.queue( xCursor::Hide )?;
        Ok(())
    }
    pub(crate) fn begin_sync() -> Result< () > {
        stdout().execute( xTerm::BeginSynchronizedUpdate )?;
        Ok(())
    }
    pub(crate) fn sync_and_flush() -> Result< () > {
        let mut stdout = stdout();
        stdout.flush()?;
        stdout.execute( xTerm::EndSynchronizedUpdate )?;
        Ok(())
    }

    pub(crate) fn clear_main_screen() -> Result< () > {
        stdout().queue( xTerm::Clear(xTerm::ClearType::All) )?;
        Ok(())
    }
    pub fn set_title(title: &str) -> Result<()> {
        stdout().execute( xTerm::SetTitle(title) )?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn size() -> Result< (u16,u16) > {
        return Ok( xTerm::size()? );
    }
    pub(crate) fn move_to( x: u16, y: u16 ) -> Result<()> {
        stdout().queue( xCursor::MoveTo( x, y) )?;
        Ok(())
    }
    pub(crate) fn print( txt: &str ) -> Result<()> {
        stdout().queue( xStyle::Print(txt) )?;
        Ok(())
    }
    pub(crate) fn set_colors( colors: xColors ) -> Result<()> {
        stdout().queue( xStyle::SetColors(colors) )?;
        Ok(())
    }
}

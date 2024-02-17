use crate::prelude::*;

//  //  //  //  //  //  //  //  //  //
use std::io::Write;
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal as xTerm;
use crossterm::cursor   as xCursor;
use crossterm::event   as  xEvent;

static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(0);
//  //  //  //  //  //  //  //  //  //
//          IMPL
//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub fn read_events() -> ResultOf< Vec<xEvent::Event> > {
        let mut result = Vec::new();
        while xEvent::poll( POLL_WAIT_TIME )? {
            result.push( xEvent::read()? );
        }
        return Ok( result );
    }
    
    pub fn info(&mut self, info: &str) {
        let _ = self.println_on_log_screen(false, info);
    }
    pub fn error(&mut self, msg: &str) {
        let _ = self.println_on_log_screen(true, msg );
    }
    fn println_on_log_screen(&mut self, is_error: bool, line: &str ) -> ResultOf< () > {
        self.restore_log_screen();
        {
            if is_error {
                eprintln!("{line}");
            }else{
                println!("{line}");
            }
        }
        self.switch_main_screen()?;
        self.flush()?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn restore_log_screen(&mut self) {
        let _ = xTerm::disable_raw_mode();
        let _ = self.stdout.execute( xEvent::DisableMouseCapture );
        let _ = self.stdout.execute( xTerm::LeaveAlternateScreen );
        let _ = Self::read_events();
        let _ = self.stdout.execute( xCursor::RestorePosition );
        let _ = self.stdout.execute( xCursor::Show );
    }
    pub(crate) fn switch_main_screen(&mut self) -> ResultOf< () > {
        xTerm::enable_raw_mode()?;
        self.stdout.queue( xCursor::SavePosition )?;
        self.stdout.queue( xTerm::EnterAlternateScreen)?;
        self.stdout.queue( xCursor::Hide )?;
        self.stdout.execute( xEvent::EnableMouseCapture )?;
        Ok(())
    }
    pub(crate) fn flush(&mut self) -> ResultOf< () > {
        self.stdout.flush()?;
        Ok(())
    }

    pub(crate) fn clean_main_screen(&mut self) -> ResultOf< () > {
        self.stdout.queue( xTerm::Clear(xTerm::ClearType::All) )?;
        Ok(())
    }
}

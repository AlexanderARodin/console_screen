use crate::prelude::*;

//  //  //  //  //  //  //  //  //  //
use std::io::Write;
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal as xTerm;
use crossterm::cursor   as xCursor;
use crossterm::style    as xStyle;

static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(0);
//  //  //  //  //  //  //  //  //  //
//          IMPL
//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub fn set_automouse_capturing( &mut self, b: bool ) {
        self.automouse_capturing = b;
    }
    pub fn get_painter( &mut self ) -> ResultOf< ConsoleDraw > {
        ConsoleDraw::new(self)
    }
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
        self.sync_and_flush()?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn restore_log_screen(&mut self) {
        let _ = xTerm::disable_raw_mode();
        let _ = self.stdout.execute( xEvent::DisableMouseCapture );
        let _ = self.stdout.execute(  xTerm::LeaveAlternateScreen );
        let _ = self.stdout.queue(    xTerm::EnableLineWrap);
        let _ = self.stdout.execute(xCursor::RestorePosition );
        let _ = self.stdout.execute(xCursor::Show );
    }
    pub(crate) fn switch_main_screen(&mut self) -> ResultOf< () > {
        xTerm::enable_raw_mode()?;
        self.stdout.execute( xTerm::BeginSynchronizedUpdate )?;
        self.stdout.queue( xCursor::SavePosition )?;
        self.stdout.queue(   xTerm::EnterAlternateScreen)?;
        self.stdout.queue(   xTerm::DisableLineWrap)?;
        self.stdout.queue( xCursor::Hide )?;
        if self.automouse_capturing {
            self.stdout.execute( xEvent::EnableMouseCapture )?;
        }
        Ok(())
    }
    pub(crate) fn begin_sync(&mut self) -> ResultOf< () > {
        self.stdout.execute( xTerm::BeginSynchronizedUpdate )?;
        Ok(())
    }
    pub(crate) fn sync_and_flush(&mut self) -> ResultOf< () > {
        self.stdout.flush()?;
        self.stdout.execute( xTerm::EndSynchronizedUpdate )?;
        Ok(())
    }

    pub(crate) fn clear_main_screen(&mut self) -> ResultOf< () > {
        self.stdout.queue( xTerm::Clear(xTerm::ClearType::All) )?;
        Ok(())
    }
    pub fn set_title(&mut self, title: &str) -> ResultOf<()> {
        self.stdout.execute( xTerm::SetTitle(title) )?;
        Ok(())
    }
}

//  //  //  //  //  //  //  //  //  //
impl ConsoleWindow {
    pub(crate) fn size() -> ResultOf< (u16,u16) > {
        return Ok( xTerm::size()? );
    }
    pub(crate) fn move_to( &mut self, x: u16, y: u16 ) -> ResultOf<()> {
        self.stdout.queue( xCursor::MoveTo( x, y) )?;
        Ok(())
    }
    pub(crate) fn print( &mut self, txt: &str ) -> ResultOf<()> {
        self.stdout.queue( xStyle::Print(txt) )?;
        Ok(())
    }
    pub(crate) fn set_colors( &mut self, colors: xColors ) -> ResultOf<()> {
        self.stdout.queue( xStyle::SetColors(colors) )?;
        Ok(())
    }
}

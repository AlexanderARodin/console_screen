use crate::prelude::*;

use crate::ConsoleWindow;


pub struct ConsoleDraw<'a> {
    console_window: &'a mut ConsoleWindow,
    pub width: u16,
    pub height: u16,
}
impl Drop for ConsoleDraw<'_> {
    fn drop(&mut self) {
        let _ = self.console_window.set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Reset)} );
        let _ = self.console_window.sync_and_flush();
    }
}

impl ConsoleDraw<'_> {
    pub(crate) fn new<'a>( console_window: &'a mut ConsoleWindow ) -> Result< ConsoleDraw > {
        let size = ConsoleWindow::size()?;
        let new_one = ConsoleDraw { 
            console_window,
            width: size.0, height: size.1,
        };
        {
            new_one.console_window.clear_main_screen()?;
            new_one.console_window.sync_and_flush()?;
            new_one.console_window.begin_sync()?;
            new_one.console_window.set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Reset)} )?;
        }
        return Ok( new_one );
    }
    
    pub fn move_to( &mut self, x: u16, y: u16 ) -> Result< &mut Self > {
        self.console_window.move_to(x, y)?;
        Ok( self )
    }
    pub fn print( &mut self, txt: &str ) -> Result< &mut Self > {
        self.console_window.print(txt)?;
        Ok( self )
    }
    pub fn set_colors( &mut self, colors: xColors ) -> Result< &mut Self > {
        self.console_window.set_colors(colors)?;
        Ok( self )
    }
    pub fn set_title(&mut self, title: &str) -> Result< &mut Self > {
        self.console_window.set_title(title)?;
        Ok( self )
    }
}


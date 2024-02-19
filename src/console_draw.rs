use crate::prelude::*;

use crate::ConsoleWindow;


pub struct ConsoleDraw {
    pub width: u16,
    pub height: u16,
}
impl Drop for ConsoleDraw {
    fn drop(&mut self) {
        let _ = ConsoleWindow::set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Reset)} );
        let _ = ConsoleWindow::sync_and_flush();
    }
}

impl ConsoleDraw {
    pub(crate) fn new( ) -> Result< Self > {
        let size = ConsoleWindow::size()?;
        ConsoleWindow::clear_main_screen()?;
        ConsoleWindow::sync_and_flush()?;
        ConsoleWindow::begin_sync()?;
        ConsoleWindow::set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Reset)} )?;
        Ok( ConsoleDraw {width: size.0, height: size.1} )
    }
    
    pub fn move_to( &mut self, x: u16, y: u16 ) -> Result< &mut Self > {
        ConsoleWindow::move_to(x, y)?;
        Ok( self )
    }
    pub fn print( &mut self, txt: &str ) -> Result< &mut Self > {
        ConsoleWindow::print(txt)?;
        Ok( self )
    }
    pub fn set_colors( &mut self, colors: xColors ) -> Result< &mut Self > {
        ConsoleWindow::set_colors(colors)?;
        Ok( self )
    }
    pub fn set_title(&mut self, title: &str) -> Result< &mut Self > {
        ConsoleWindow::set_title(title)?;
        Ok( self )
    }
}


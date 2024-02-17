use crate::prelude::*;

use crate::ConsoleWindow;


pub struct ConsoleDraw<'a> {
    console_window: &'a mut ConsoleWindow,
}
impl Drop for ConsoleDraw<'_> {
    fn drop(&mut self) {
        self.console_window.sync_and_flush();
    }
}

impl ConsoleDraw<'_> {
    pub fn new<'a>( console_window: &'a mut ConsoleWindow ) -> ResultOf< ConsoleDraw > {
        let new_one = ConsoleDraw { console_window };
        {
            new_one.console_window.clear_main_screen();
            new_one.console_window.sync_and_flush();
            new_one.console_window.begin_sync()?;
        }
        return Ok( new_one );
    }
    
    pub fn move_to( &mut self, x: u16, y: u16 ) -> ResultOf< &mut Self > {
        self.console_window.move_to(x, y)?;
        Ok( self )
    }
    pub fn print( &mut self, txt: &str ) -> ResultOf< &mut Self> {
        self.console_window.print(txt)?;
        Ok( self )
    }
}


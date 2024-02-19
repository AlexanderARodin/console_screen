pub mod prelude;
    use prelude::*;



//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
mod impl_console_window;
mod console_draw;

pub struct ConsoleWindow;
impl Drop for ConsoleWindow {
    fn drop(&mut self) {
        Self::restore_log_screen();
        println!( "<-- ConsoleWindow destroyed" );
    }
}

impl ConsoleWindow {
    pub fn new() -> Result< Self > {
        println!( "-> ConsoleWindow preparing.." );
        crossterm::terminal::enable_raw_mode()?;
        Self::switch_main_screen()?;
        Self::clear_main_screen()?;
        Self::info( "--> ConsoleWindow created" );
        Ok( Self )
    }
}

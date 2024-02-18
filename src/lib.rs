pub mod prelude;
    use prelude::*;



//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
mod impl_console_window;
mod console_draw;

pub struct ConsoleWindow {
    stdout: std::io::Stdout,
    automouse_capturing: bool,
}
impl Drop for ConsoleWindow {
    fn drop(&mut self) {
        self.restore_log_screen();
        println!( "<-- ConsoleWindow destroyed" );
    }
}

impl ConsoleWindow {
    pub fn new() -> Result< Self > {
        println!( "-> ConsoleWindow preparing.." );
        crossterm::terminal::enable_raw_mode()?;
        let stdout = std::io::stdout();
        let mut new_one= Self{stdout,automouse_capturing:false};
            {
                new_one.switch_main_screen()?;
                new_one.clear_main_screen()?;
                new_one.info( "--> ConsoleWindow created" );
            }
        Ok( new_one )
    }
}

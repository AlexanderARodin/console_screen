pub mod prelude;
    use prelude::*;


//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
mod impl_console_window;

pub struct ConsoleWindow {
    stdout: std::io::Stdout,
}
impl Drop for ConsoleWindow {
    fn drop(&mut self) {
        self.restore_log_screen();
        println!( "<-- ConsoleWindow destroyed" );
    }
}

impl ConsoleWindow {
    pub fn new() -> ResultOf< Self > {
        println!( "-> ConsoleWindow preparing.." );
        crossterm::terminal::enable_raw_mode()?;
        let stdout = std::io::stdout();
        let mut new_one= Self{stdout};
            {
                new_one.switch_main_screen()?;
                new_one.clean_main_screen()?;
                new_one.info( "--> ConsoleWindow created" );
            }
        Ok( new_one )
    }
}

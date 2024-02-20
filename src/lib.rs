pub mod prelude;

    use prelude::*;



//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
mod impl_console_window;
mod console_draw;

pub struct ConsoleWindow {
    state: ConsoleWindowState,
}
pub enum ConsoleWindowState {
    NotTerminal,
    Main,
    Alt(bool),
}

impl Drop for ConsoleWindow {
    fn drop(&mut self) {
        if let ConsoleWindowState::NotTerminal = self.state {
            println!( "<-- ConsoleWindow destroyed (NotTerminal)" );
        }else{
            if let Err(e) = self.restore_main_screen() {
                eprintln!( "{}", e.to_string() );
            }
            println!( "<-- ConsoleWindow destroyed" );
        }
    }
}

impl ConsoleWindow {
    pub fn new() -> Result<Self> {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            Ok( Self { state: ConsoleWindowState::Main } )
        }else{
            Ok( Self { state: ConsoleWindowState::NotTerminal } )
        }
    }
}


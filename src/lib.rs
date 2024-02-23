pub mod prelude;

use prelude::*;

//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
mod console_draw;
mod impl_console_window;

pub enum ScreenState {
    NotTerminal,
    Main,
    Alt(bool),
}

//##//##//##//##//##//##//##//##//##//##//##//##//##
use anyhow::anyhow;
use std::io::Write;

pub struct ScreenCore {
    state: ScreenState,
}

impl ScreenCore {
    pub fn new() -> Result<Self> {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            p("<-- ScreenState::Main");
            Ok(Self {
                state: ScreenState::Main,
            })
        } else {
            p("<-- ScreenState::NotTerminal");
            Ok(Self {
                state: ScreenState::NotTerminal,
            })
        }
    }
}

impl Drop for ScreenCore {
    fn drop(&mut self) {
        p("<-- ScreenState destroyed");
    }
}

fn p(t: &str) {
    std::io::stdout().flush().unwrap();
    println!("# {}", t);
    std::io::stdout().flush().unwrap();
}

impl ScreenCore {
    pub fn get_state(&self) -> &ScreenState {
        &self.state
    }
}
//##//##//##//##//##//##//##//##//##//##//##//##//##
pub enum ConsoleWindowState {
    NotTerminal,
    Main,
    Alt(bool),
}

pub struct ConsoleWindow {
    state: ConsoleWindowState,
}

impl ConsoleWindow {
    pub fn new() -> Result<Self> {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            Ok(Self {
                state: ConsoleWindowState::Main,
            })
        } else {
            Ok(Self {
                state: ConsoleWindowState::NotTerminal,
            })
        }
    }
}

impl Drop for ConsoleWindow {
    fn drop(&mut self) {
        if let ConsoleWindowState::NotTerminal = self.state {
            println!("<-- ConsoleWindow destroyed (NotTerminal)");
        } else {
            if let Err(e) = self.restore_main_screen() {
                eprintln!("{}", e.to_string());
            }
            println!("<-- ConsoleWindow destroyed");
        }
    }
}

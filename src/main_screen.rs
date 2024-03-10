use anyhow::anyhow;
use anyhow::Result;

use std::io::Write;

//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
use crate::alt_screen::*;

pub struct MainScreen {}

impl MainScreen {
    pub fn new() -> Result<Self> {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            p("<-- MainScreen::new");
            return Ok(Self {});
        } else {
            return Err(anyhow!("it's not a terminal"));
        }
    }
    pub fn print(&self, s: &str) {
        p(s);
    }
    pub fn go_alt_screen(self) -> Result<AltScreen> {
        drop(self);
        AltScreen::new(false)
    }
}

impl Drop for MainScreen {
    fn drop(&mut self) {
        p("<-- Drop MainScreen");
    }
}

//##//##//##//##//##//##//##//##//##//##//##//##//##
fn p(t: &str) {
    std::io::stdout().flush().unwrap();
    println!("# {}", t);
    std::io::stdout().flush().unwrap();
}
//##//##//##//##//##//##//##//##//##//##//##//##//##

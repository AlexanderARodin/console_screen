pub mod prelude;




//  //  //  //  //  //  //  //  //  //
//          CORE
//  //  //  //  //  //  //  //  //  //
use std::io::{Stdout,Write};

pub(crate) struct ConsoleWindow {
    stdout: Stdout,
}
impl Drop for ConsoleWindow {
    fn drop(&mut self) {
    }
}

impl ConsoleWindow {
    //
}

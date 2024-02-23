use std::process::exit;

use console_window::prelude::*;

fn main() {
    println!("\n--> let's check it..\n");

    match wrapper() {
        Err(e) => {
            eprintln!("E: {}", e.to_string());
            exit(-1);
        }
        Ok(_) => {}
    }
    println!("\n<-- ..FIN!\n");
}

fn wrapper() -> Result<()> {
    //
    let mut ss = ScreenCore::new()?;
    std::thread::sleep(std::time::Duration::from_millis(333)); // TODO: debug only
    Ok(())
}

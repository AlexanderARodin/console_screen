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
    let mut cw = ConsoleWindow::new()?;
    //cw.restore_main_screen()?;
    cw.info("one two");
    std::thread::sleep(std::time::Duration::from_millis(999)); // TODO: debug only
    cw.info("1 2 3 4");
    std::thread::sleep(std::time::Duration::from_millis(999)); // TODO: debug only
    cw.error("0 9 8 7");
    std::thread::sleep(std::time::Duration::from_millis(999)); // TODO: debug only
    cw.enter_alt_screen(false)?;
    println!("xxxxxxxx");
    std::thread::sleep(std::time::Duration::from_millis(3333)); // TODO: debug only
    cw.error("6969");
    std::thread::sleep(std::time::Duration::from_millis(999)); // TODO: debug only
    cw.enter_alt_screen(false)?;
    println!("uuuuuuu");
    std::thread::sleep(std::time::Duration::from_millis(3333)); // TODO: debug only
    cw.info("oops");
    std::thread::sleep(std::time::Duration::from_millis(3333)); // TODO: debug only
    cw.restore_main_screen()?;
    cw.info("Ho-ho-hooo");
    std::thread::sleep(std::time::Duration::from_millis(3333)); // TODO: debug only
    Ok(())
}

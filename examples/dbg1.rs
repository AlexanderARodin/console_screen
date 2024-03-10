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
    let ss = MainScreen::new()?;
    for i in 0..6 {
        ss.print(&i.to_string());
        std::thread::sleep(std::time::Duration::from_millis(222));
    }
    let alt_screen = ss.go_alt_screen()?;
    for i in 0..10 {
        println!("->{}<-", i);
        std::thread::sleep(std::time::Duration::from_millis(222));
    }
    std::thread::sleep(std::time::Duration::from_millis(555));
    //
    let ss2 = alt_screen.go_main_screen()?;
    for i in 6..10 {
        ss2.print(&i.to_string());
        std::thread::sleep(std::time::Duration::from_millis(222));
    }
    ss2.print("fine!!!");
    Ok(())
}

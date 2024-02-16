use std::process::exit;

use console_window::prelude::*;


fn main() {
    println!( "\n--> let's check it..\n" );

    match wrapper() {
        Err(e) => {
            eprintln!("E: <{}>", e.to_string() );
            exit(-1);
        },
        Ok(_) => {
        },
    }
    println!( "\n<-- ..FIN!\n" );
}

fn wrapper() -> ResultOf<()> {
    let mut cw = ConsoleWindow::new()?;
    
    cw.info("info 1");
    println!( "#        test         #" );
    std::thread::sleep(std::time::Duration::from_millis(1000));
    cw.info("info 22");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!( "#        t e s t 33         #" );
    std::thread::sleep(std::time::Duration::from_millis(1000));
    cw.error("error 1");
    Ok(())
}


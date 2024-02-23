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
    Ok(())
}

/*
fn wrapper_old() -> Result<()> {
    let cw = ConsoleWindow::new()?;
    cw.info("enter loop..");
    let mut pointer = (0,0);
    for i in 0..= 10000 {//65535 {
        let title = format!(" --> {}", i);
        ConsoleWindow::set_title(&title)?;
        match process_input()? {
            None => {},
            Some( pos ) => {
                pointer = pos;
            },
        }
        {
            process_draw( &mut ConsoleWindow::get_painter()?, i, &pointer )?;
            ConsoleWindow::capture_mouse()?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: debug only
    }
    ConsoleWindow::info("exit loop");
    Ok(())
}

fn process_input() -> Result< Option<(u16,u16)> > {
    let inputs = ConsoleWindow::read_events()?;
    let mut result: Option< (u16,u16) > = None;
    for event in inputs {
        match event {
            xEvent::Event::Key(key) => {
                if key.code == xEvent::KeyCode::Char('g') {
                    ConsoleWindow::info("GGGGGGGG");
                    ConsoleWindow::info("hhhhhhhh");
        std::thread::sleep(std::time::Duration::from_millis(1000)); // TODO: debug only
                }
                if key.code == xEvent::KeyCode::Char('c') {
                    if key.modifiers .contains( xEvent::KeyModifiers::CONTROL ) {
                        return Err(anyhow!( "<C-c>" ));
                    }
                }else if key.code == xEvent::KeyCode::Esc {
                    return Err(anyhow!( "Esc" ));
                }
            },
            xEvent::Event::Mouse( mouse_event ) => {
               result = Some( (mouse_event.column, mouse_event.row) );
            },
            _ => {
            },
        }
    }
    Ok( result )
}

fn process_draw( cd: &mut ConsoleDraw, i: u16, pointer: &(u16,u16) ) -> Result<()> {
    cd  .move_to( i/100, i/100 )?
        .print( "x---------------------------------------------------------x" )?;
    //
    cd  .set_colors( xColors{foreground:Some(xColor::Black),background:Some(xColor::Blue)} )?;
    //
    cd  .move_to( pointer.0, pointer.1 )?
        .print("+")?;
    if pointer.0 >= 5 {
        cd  .move_to( pointer.0 - 5, pointer.1 )?
            .print(">")?;
    }
    if (pointer.0+5) < cd.width {
        cd  .move_to( pointer.0 + 5, pointer.1 )?
            .print("<")?;
    }
    if (pointer.1+1) < cd.height {
        cd  .move_to( pointer.0, pointer.1+1 )?
            .print("^")?;
    }
    //
    cd  .set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Grey)} )?;
    let info = format!( "size: {},{}", cd.width, cd.height);
    cd  .move_to( 10, 10 )?
        .set_colors( xColors{foreground:None,background:None} )?
        .print(&info)?;
    let info2 = format!( "cursor: {},{}", pointer.0, pointer.1);
    cd  .move_to( 10, 11 )?
        .print(&info2)?;
    Ok(())
}
*/

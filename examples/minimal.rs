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
    cw.set_automouse_capturing(true);
    cw.info("enter loop..");
    let mut pointer = (0,0);
    for i in 0..=65535 {
        match process_input( &mut cw )? {
            None => {},
            Some( pos ) => {
                pointer = pos;
            },
        }
        {
            process_draw( &mut cw.getPainter()?, i, &pointer )?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: debug only
    }
    cw.info("exit loop");
    Ok(())
}

fn process_input( cw: &mut ConsoleWindow ) -> ResultOf< Option<(u16,u16)> > {
    let inputs = ConsoleWindow::read_events()?;
    let mut result: Option< (u16,u16) > = None;
    for event in inputs {
        match event {
            xEvent::Event::Key(key) => {
                if key.code == xEvent::KeyCode::Char('c') {
                    if key.modifiers .contains( xEvent::KeyModifiers::CONTROL ) {
                        return Err(Box::from( "<C-c>" ));
                    }
                }else if key.code == xEvent::KeyCode::Esc {
                    return Err(Box::from( "Esc" ));
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

fn process_draw( cd: &mut ConsoleDraw, i: u16, pointer: &(u16,u16) ) -> ResultOf<()> {
    cd  .move_to( i, i )?
        .print( "GGGG--------------------------------------------GG" )?;
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
    let info = format!( "size: {},{}", cd.width, cd.height);
    cd  .move_to( 10, 10 )?
        .print(&info)?;
    let info2 = format!( "cursor: {},{}", pointer.0, pointer.1);
    cd  .move_to( 10, 11 )?
        .print(&info2)?;
    Ok(())
}

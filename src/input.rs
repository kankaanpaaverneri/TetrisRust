use std::io;

use crossterm::event::{read, Event, KeyCode};

#[derive(Debug)]
pub enum InputCommand {
    Left,
    Right,
    Down,
    Rotate,
    Exit,
    None,
}

pub fn read_user_input() -> io::Result<InputCommand> {
    let read_result = read();
    if let Ok(event) = read_result {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Down => return Ok(InputCommand::Down),
                KeyCode::Left => return Ok(InputCommand::Left),
                KeyCode::Right => return Ok(InputCommand::Right),
                KeyCode::Char(character) => {
                    if character == 'r' {
                        return Ok(InputCommand::Rotate);
                    }
                    return Ok(InputCommand::None);
                }
                KeyCode::Esc => return Ok(InputCommand::Exit),
                _ => return Ok(InputCommand::None),
            },
            _ => return Ok(InputCommand::None),
        }
    } else if let Err(error) = read_result {
        eprint!("Error when reading result: {}\r\n", error);
        if let Err(raw_mode_error) = crossterm::terminal::disable_raw_mode() {
            print!("Failed to disable raw_mode: {}\r\n", raw_mode_error);
        }
        std::process::exit(1);
    }
    Ok(InputCommand::None)
}

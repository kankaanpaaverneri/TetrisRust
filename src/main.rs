use std::{fs::File, io, time::Duration};
mod board;
mod input;
mod tetromino;

use board::{
    clear_tetromino, collapse, display_board, game_over, init_board, move_to_direction,
    tetromino_status, write_tetromino_to_board, GameStatus, MoveDirection,
};
use crossterm::{
    event::poll,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize},
};
use input::InputCommand;
use tetromino::{get_random_tetromino, rotate_tetromino, ZIG1, ZIG2};

use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::BufReader;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let (_output_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|error| {
        eprint!("Could not get OutputStream: {}\r\n", error);
        std::process::exit(1);
    });
    let file = File::open("tetris_theme.mp3").unwrap_or_else(|error| {
        eprint!("Could not open audio file: {}\r\n", error);
        std::process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    let decoder = Decoder::new(buf_reader).unwrap_or_else(|error| {
        eprint!("Could not create decoder: {}\r\n", error);
        std::process::exit(1);
    });
    let sink = Sink::try_new(&stream_handle).unwrap_or_else(|error| {
        eprint!("Could not create Sink: {}\r\n", error);
        std::process::exit(1);
    });
    sink.append(decoder.repeat_infinite());
    sink.play();
    execute!(io::stdout(), SetSize(100, 25), Clear(ClearType::All))?;
    let mut board = init_board();
    let mut exit = false;
    let mut points: usize = 0;
    let mut time_in_milliseconds = 500;
    let mut sound_playback_speed = 1.0;
    let mut tetrominos_dropped = 0;
    while !exit {
        let mut tetromino_type = get_random_tetromino();
        if (tetromino_type == ZIG1 || tetromino_type == ZIG2) && tetrominos_dropped == 0 {
            continue;
        }
        let mut tetromino_y_position = 0;
        let mut tetromino_x_position = 4;
        let mut tetromino_positions = Vec::with_capacity(0);
        loop {
            execute!(io::stdout(), Clear(ClearType::FromCursorDown))?;
            if poll(Duration::from_millis(time_in_milliseconds))? {
                let input = input::read_user_input()?;
                //print!("input: {:?}\r\n", input);
                match input {
                    InputCommand::Left => {
                        tetromino_x_position = move_to_direction(
                            &tetromino_positions,
                            &board,
                            tetromino_x_position,
                            MoveDirection::Left,
                        );
                    }
                    InputCommand::Right => {
                        tetromino_x_position = move_to_direction(
                            &tetromino_positions,
                            &board,
                            tetromino_x_position,
                            MoveDirection::Right,
                        );
                    }
                    InputCommand::Exit => {
                        exit = true;
                        break;
                    }
                    InputCommand::Down => {}
                    InputCommand::Rotate => {
                        rotate_tetromino(&mut tetromino_type);
                    }
                    InputCommand::None => {}
                }
            }
            //print!("x: {tetromino_x_position}, y: {tetromino_y_position}\r\n");
            if tetromino_x_position < 0 {
                tetromino_x_position = 1;
            }
            tetromino_positions = write_tetromino_to_board(
                &mut board,
                tetromino_y_position,
                tetromino_x_position,
                tetromino_type,
            );

            display_board(&mut board, points);
            match tetromino_status(&board, &tetromino_positions) {
                GameStatus::TetrominoBottomCollision => {
                    break;
                }
                GameStatus::TetrominoDropping => {
                    execute!(io::stdout(), Clear(ClearType::FromCursorDown))?;
                    clear_tetromino(&mut board, &tetromino_positions);
                    tetromino_y_position += 1;
                }
            }
        }
        if collapse(&mut board) {
            points += 1;
            if time_in_milliseconds > 50 {
                time_in_milliseconds -= 50;
            }

            sound_playback_speed += 0.1;
            sink.set_speed(sound_playback_speed);
        }
        if game_over(&board) {
            print!("Game Over\r\n");
            break;
        }
        tetrominos_dropped += 1;
    }

    disable_raw_mode()?;
    Ok(())
}

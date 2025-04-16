use std::io;

use crossterm::event::{read, Event, KeyCode};

use crate::{
    board::{
        clear_previous_tetromino_from_board, copy_tetromino_positions, get_tetromino_positions,
        is_tetromino_position_overlapping_with_block, is_tetromino_positions_out_of_bounds,
        move_tetromino_back_in_bounds, move_tetromino_sideways, Coordinate, MoveDirection,
        BOARD_HEIGHT, BOARD_WIDTH,
    },
    tetromino::{
        is_left_of_tetromino_empty, is_top_of_tetromino_empty, move_tetromino_left_by_one,
        move_tetromino_up_by_one, rotate_tetromino, TETROMINO_SIZE,
    },
};

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

pub fn commit_action(
    input: &InputCommand,
    tetromino_positions: &mut [Coordinate; TETROMINO_SIZE],
    tetromino: &mut [[char; TETROMINO_SIZE]; TETROMINO_SIZE],
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
) -> bool {
    match input {
        InputCommand::Left => {
            let previous_tetromino_positions = copy_tetromino_positions(&tetromino_positions);
            if move_tetromino_sideways(tetromino_positions, board, &MoveDirection::Left) {
                clear_previous_tetromino_from_board(&previous_tetromino_positions, board);
            }
        }
        InputCommand::Right => {
            let previous_tetromino_positions = copy_tetromino_positions(&tetromino_positions);
            if move_tetromino_sideways(tetromino_positions, board, &MoveDirection::Right) {
                clear_previous_tetromino_from_board(&previous_tetromino_positions, board);
            }
        }
        InputCommand::Exit => {
            return false;
        }
        InputCommand::Down => {}
        InputCommand::Rotate => {
            let previous_tetromino_positions = copy_tetromino_positions(&tetromino_positions);
            rotate_tetromino(tetromino);

            // Prevents tetrominos moving when rotating
            // Move tetromino back to up position after rotation
            while is_top_of_tetromino_empty(tetromino) {
                move_tetromino_up_by_one(tetromino);
            }
            // Move tetromino back to left position after rotation
            while is_left_of_tetromino_empty(tetromino) {
                move_tetromino_left_by_one(tetromino);
            }
            clear_previous_tetromino_from_board(&previous_tetromino_positions, board);
            let mut updated_tetromino_positions =
                get_tetromino_positions(tetromino, tetromino_positions);
            let out_of_bounds = is_tetromino_positions_out_of_bounds(&updated_tetromino_positions);
            if out_of_bounds.get_difference() > 0 {
                move_tetromino_back_in_bounds(&out_of_bounds, &mut updated_tetromino_positions);
            }
            for i in 0..TETROMINO_SIZE {
                tetromino_positions[i] = updated_tetromino_positions[i];
            }

            // Check if updated_tetromino_positions is overlapping with block
            if is_tetromino_position_overlapping_with_block(tetromino_positions, board) {
                for i in 0..TETROMINO_SIZE {
                    tetromino_positions[i] = previous_tetromino_positions[i];
                }
            }
        }
        InputCommand::None => {}
    }
    true
}

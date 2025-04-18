use std::time::Duration;

use crossterm::{
    event::poll,
    execute,
    terminal::{Clear, ClearType, SetSize},
};

use crate::{
    audio::Audio,
    board::{
        clear_previous_tetromino_from_board, collapse_full_row, copy_tetromino_positions,
        drop_tetromino, game_over, is_full_row, write_tetromino_to_board,
    },
    input::commit_action,
    tetromino::{get_random_tetromino, SBAR1, SBAR2},
};
use crate::{
    board::{display_board, init_tetromino_to_board},
    input::read_user_input,
};

use crate::board::init_board;

pub fn start_game(audio: Option<&Audio>) -> std::io::Result<()> {
    execute!(std::io::stdout(), SetSize(100, 25), Clear(ClearType::All))?;
    let mut board = init_board();
    let mut exit = false;
    let mut points: usize = 0;
    let mut time_in_milliseconds = 1000;
    let mut sound_playback_speed = 1.0;
    let mut tetrominos_dropped = 0;
    while !exit {
        let mut tetromino = get_random_tetromino();
        if (tetromino == SBAR1 || tetromino == SBAR2) && tetrominos_dropped == 0 {
            continue;
        }
        let mut tetromino_positions = init_tetromino_to_board(&mut board, &tetromino, 4, 0);

        loop {
            if poll(Duration::from_millis(time_in_milliseconds))? {
                let input = read_user_input()?;
                if !commit_action(&input, &mut tetromino_positions, &mut tetromino, &mut board) {
                    exit = true;
                    break;
                }
            }

            let prev_tetromino_positions = copy_tetromino_positions(&tetromino_positions);
            if !drop_tetromino(&mut tetromino_positions, &board) {
                write_tetromino_to_board(&tetromino_positions, &mut board);
                display_board(&board);
                break;
            }

            clear_previous_tetromino_from_board(&prev_tetromino_positions, &mut board);
            write_tetromino_to_board(&tetromino_positions, &mut board);

            display_board(&board);

            execute!(std::io::stdout(), Clear(ClearType::FromCursorDown))?;
        }
        let mut column = is_full_row(&board);
        if column != 0 {
            sound_playback_speed += 0.1;
            if let Some(audio) = audio {
                audio.get_sink().set_speed(sound_playback_speed);
            }
            if time_in_milliseconds > 100 {
                time_in_milliseconds -= 100;
            }
            points += 1;
        }
        while column != 0 {
            collapse_full_row(&mut board, &column);
            column = is_full_row(&board);
            points += 1;
        }
        if game_over(&tetromino_positions) && tetrominos_dropped > 0 {
            print!("GAME OVER\r\n");
            print!("Points: {}\r\n", points);
            exit = true;
        }
        tetrominos_dropped += 1;
    }
    Ok(())
}

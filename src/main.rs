mod audio;
mod board;
mod game;
mod input;
mod tetromino;

use audio::play_audio;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use game::start_game;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    match play_audio() {
        Ok(audio) => start_game(Some(&audio))?,

        Err(error) => {
            eprint!("Could not play audio: {}\r\n", error);

            start_game(None)?;
        }
    }

    disable_raw_mode()?;
    Ok(())
}

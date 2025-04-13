use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub const BLOCK: char = '#';
pub const EMPTY: char = ' ';

pub const TETROMINO_SIZE: usize = 4;

pub const ZIG1: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [EMPTY, BLOCK, EMPTY, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, EMPTY, BLOCK, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];
pub const ZIG2: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [EMPTY, EMPTY, BLOCK, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, BLOCK, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];

pub const LBAR1: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [EMPTY, BLOCK, EMPTY, EMPTY],
    [EMPTY, BLOCK, EMPTY, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];

pub const LBAR2: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [EMPTY, EMPTY, BLOCK, EMPTY],
    [EMPTY, EMPTY, BLOCK, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];

pub const TBAR: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [BLOCK, BLOCK, BLOCK, EMPTY],
    [EMPTY, BLOCK, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];

pub const LINE: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [BLOCK, BLOCK, BLOCK, BLOCK],
    [EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];
pub const SQUARE: [[char; TETROMINO_SIZE]; TETROMINO_SIZE] = [
    [EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, BLOCK, BLOCK, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY],
];

pub fn get_random_tetromino() -> [[char; TETROMINO_SIZE]; TETROMINO_SIZE] {
    let tetrominos = [ZIG1, ZIG2, LBAR1, LBAR2, TBAR, LINE, SQUARE];
    let result = generate_random_number();
    match result {
        Ok(random_index) => {
            return tetrominos[random_index];
        }
        Err(error) => {
            eprint!("Failed to generate random number: {}\r\n", error);
            std::process::exit(1)
        }
    }
}

fn generate_random_number() -> Result<usize, SystemTimeError> {
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH);
    match system_time {
        Ok(time) => {
            let pointer = &time.as_secs();
            let random_value = pointer + std::process::id() as u64 % 7;
            return Ok((random_value % 7) as usize);
        }
        Err(error) => {
            return Err(error);
        }
    }
}

pub fn rotate_tetromino(tetromino: &mut [[char; 4]; 4]) {
    let tetromino_copy = tetromino.clone();
    let mut y = 0;

    for i in (0..TETROMINO_SIZE).rev() {
        let mut x = 0;
        for j in 0..TETROMINO_SIZE {
            tetromino[y][x] = tetromino_copy[j][i];
            x += 1;
        }
        y += 1;
    }
}

//pub fn rotate_back_to_previous(tetromino: &mut [[char; 4]; 4]) {
//    let tetromino_copy = tetromino.clone();
//    let mut y = 0;
//
//    for i in 0..TETROMINO_SIZE {
//        let mut x = 0;
//        for j in 0..TETROMINO_SIZE {
//            tetromino[y][x] = tetromino_copy[j][i];
//            x += 1;
//        }
//        y += 1;
//    }
//}

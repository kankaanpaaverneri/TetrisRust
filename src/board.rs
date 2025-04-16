use crate::tetromino::TETROMINO_SIZE;

const EMPTY: char = ' ';
const BLOCK: char = '#';

pub const BOARD_WIDTH: usize = 12;
pub const BOARD_HEIGHT: usize = 22;

pub fn init_board() -> [[char; BOARD_WIDTH]; BOARD_HEIGHT] {
    let mut board = [[EMPTY; BOARD_WIDTH]; BOARD_HEIGHT];
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            if i == BOARD_HEIGHT - 1 {
                board[i][j] = BLOCK;
            }
            if j == 0 || j == BOARD_WIDTH - 1 {
                board[i][j] = BLOCK;
            }
        }
    }
    board
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    y: isize,
    x: isize,
}

impl Coordinate {
    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn get_y(&self) -> isize {
        self.y
    }
}

pub enum MoveDirection {
    Left,
    Right,
    Down,
    Up,
    None,
}

pub fn init_tetromino_to_board(
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
    tetromino: &[[char; TETROMINO_SIZE]; TETROMINO_SIZE],
    start_x: isize,
    start_y: isize,
) -> [Coordinate; TETROMINO_SIZE] {
    let mut tetromino_positions = [
        Coordinate { y: 0, x: 0 },
        Coordinate { y: 0, x: 0 },
        Coordinate { y: 0, x: 0 },
        Coordinate { y: 0, x: 0 },
    ];
    let mut index = 0;
    for i in 0..TETROMINO_SIZE {
        for j in 0..TETROMINO_SIZE {
            if tetromino[i][j] == BLOCK && start_y >= 0 && start_x < (BOARD_WIDTH - 2) as isize {
                board[i + start_y as usize][j + start_x as usize] = tetromino[i][j];
                tetromino_positions[index].x = j as isize + start_x;
                tetromino_positions[index].y = i as isize + start_y;
                index += 1;
            }
        }
    }
    tetromino_positions
}

pub fn display_board(board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!("{} ", board[i][j]);
        }
        print!("\r\n");
    }
}

pub fn drop_tetromino(
    tetromino_positions: &mut [Coordinate; TETROMINO_SIZE],
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
) -> bool {
    if is_allowed_to_drop_tetromino(tetromino_positions, board) {
        for i in 0..TETROMINO_SIZE {
            tetromino_positions[i].y += 1;
        }
        return true;
    }
    false
}

pub fn clear_previous_tetromino_from_board(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
) {
    for i in 0..TETROMINO_SIZE {
        board[tetromino_positions[i].y as usize][tetromino_positions[i].x as usize] = EMPTY;
    }
}

pub fn copy_tetromino_positions(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
) -> [Coordinate; TETROMINO_SIZE] {
    let mut new_tetromino_positions = [
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
    ];

    for i in 0..TETROMINO_SIZE {
        new_tetromino_positions[i] = tetromino_positions[i];
    }
    new_tetromino_positions
}

pub fn write_tetromino_to_board(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
) {
    for i in 0..TETROMINO_SIZE {
        board[tetromino_positions[i].y as usize][tetromino_positions[i].x as usize] = BLOCK;
    }
}

pub fn move_tetromino_sideways(
    tetromino_positions: &mut [Coordinate; TETROMINO_SIZE],
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    direction: &MoveDirection,
) -> bool {
    let direction_is_empty = false;
    match direction {
        MoveDirection::Left => {
            let direction_is_empty = is_direction_empty(tetromino_positions, board, -1);
            if direction_is_empty {
                move_to_direction(tetromino_positions, -1);
            }
            return direction_is_empty;
        }
        MoveDirection::Right => {
            let direction_is_empty = is_direction_empty(tetromino_positions, board, 1);
            if direction_is_empty {
                move_to_direction(tetromino_positions, 1);
            }
            return direction_is_empty;
        }
        MoveDirection::Down => {
            return direction_is_empty;
        }
        _ => direction_is_empty,
    }
}

pub fn get_tetromino_positions(
    tetromino: &[[char; TETROMINO_SIZE]; TETROMINO_SIZE],
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
) -> [Coordinate; TETROMINO_SIZE] {
    let start_x = tetromino_positions[0].get_x();
    let start_y = tetromino_positions[0].get_y();
    let mut updated_tetromino_positions = [
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 0 },
    ];
    let mut index = 0;
    for i in 0..TETROMINO_SIZE {
        for j in 0..TETROMINO_SIZE {
            if tetromino[i][j] == BLOCK {
                updated_tetromino_positions[index].y = start_y + i as isize;
                updated_tetromino_positions[index].x = start_x + j as isize;
                index += 1;
            }
        }
    }
    updated_tetromino_positions
}

pub struct OutOfBounds {
    difference: isize,
    direction: MoveDirection,
}

impl OutOfBounds {
    pub fn get_difference(&self) -> isize {
        self.difference
    }
}

pub fn is_tetromino_positions_out_of_bounds(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
) -> OutOfBounds {
    let max_x: isize = (BOARD_WIDTH - 1) as isize;
    let min_x: isize = 0;
    let max_y: isize = (BOARD_HEIGHT - 1) as isize;
    let min_y: isize = 0;
    let mut out_of_bounds = OutOfBounds {
        difference: 0,
        direction: MoveDirection::None,
    };
    for i in 0..TETROMINO_SIZE {
        if tetromino_positions[i].x > max_x {
            out_of_bounds.difference = tetromino_positions[i].x - max_x;
            out_of_bounds.direction = MoveDirection::Right;
        } else if tetromino_positions[i].x < min_x {
            out_of_bounds.difference = (min_x - tetromino_positions[i].x).abs();
            out_of_bounds.direction = MoveDirection::Left;
        } else if tetromino_positions[i].y > max_y {
            out_of_bounds.difference = tetromino_positions[i].y - max_y;
            out_of_bounds.direction = MoveDirection::Down;
        } else if tetromino_positions[i].y < min_y {
            out_of_bounds.difference = (min_y - tetromino_positions[i].y).abs();
            out_of_bounds.direction = MoveDirection::Up;
        }
    }
    out_of_bounds
}

pub fn move_tetromino_back_in_bounds(
    out_of_bounds: &OutOfBounds,
    tetromino_positions: &mut [Coordinate; TETROMINO_SIZE],
) {
    match out_of_bounds.direction {
        MoveDirection::Up => {
            for i in 0..TETROMINO_SIZE {
                tetromino_positions[i].y += out_of_bounds.difference + 1;
            }
        }
        MoveDirection::Left => {
            for i in 0..TETROMINO_SIZE {
                tetromino_positions[i].x += out_of_bounds.difference + 1;
            }
        }
        MoveDirection::Right => {
            for i in 0..TETROMINO_SIZE {
                tetromino_positions[i].x -= out_of_bounds.difference + 1;
            }
        }
        MoveDirection::Down => {
            for i in 0..TETROMINO_SIZE {
                tetromino_positions[i].y -= out_of_bounds.difference + 1;
            }
        }
        _ => {}
    }
}

pub fn is_tetromino_position_overlapping_with_block(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
) -> bool {
    for i in 0..TETROMINO_SIZE {
        if board[tetromino_positions[i].y as usize][tetromino_positions[i].x as usize] == BLOCK {
            return true;
        }
    }
    false
}

pub fn is_full_row(board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT]) -> usize {
    let mut column_with_full_row = 0;
    for i in 1..BOARD_HEIGHT - 1 {
        let mut block_counter = 0;
        for j in 1..BOARD_WIDTH - 1 {
            if board[i][j] == BLOCK {
                block_counter += 1;
            }
        }
        if block_counter == 10 {
            column_with_full_row = i;
        }
    }
    column_with_full_row
}

pub fn collapse_full_row(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT], column: &usize) {
    for i in (1..column + 1).rev() {
        for j in 1..BOARD_WIDTH - 1 {
            board[i][j] = board[i - 1][j];
        }
    }
}

pub fn game_over(tetromino_positions: &[Coordinate; TETROMINO_SIZE]) -> bool {
    for i in 0..TETROMINO_SIZE {
        if tetromino_positions[i].y <= 0 {
            return true;
        }
    }
    false
}

fn is_direction_empty(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    direction_value: isize,
) -> bool {
    let mut allowed_to_move = false;
    let direction = match direction_value {
        -1 => MoveDirection::Left,
        1 => MoveDirection::Right,
        _ => MoveDirection::Down,
    };
    for i in 0..TETROMINO_SIZE {
        if !is_own_coordinate(&tetromino_positions[i], tetromino_positions, &direction) {
            let mut board_position = board[tetromino_positions[i].y as usize]
                [(tetromino_positions[i].x + direction_value) as usize];
            if direction_value < 0 {
                board_position = board[tetromino_positions[i].y as usize]
                    [(tetromino_positions[i].x - direction_value.abs()) as usize];
            }
            if board_position == BLOCK {
                return false;
            } else {
                allowed_to_move = true;
            }
        }
    }
    allowed_to_move
}

fn move_to_direction(tetromino_positions: &mut [Coordinate; TETROMINO_SIZE], direction: isize) {
    for i in 0..TETROMINO_SIZE {
        tetromino_positions[i].x += direction;
    }
}

fn is_allowed_to_drop_tetromino(
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
) -> bool {
    let mut allowed_to_drop = false;
    for i in 0..TETROMINO_SIZE {
        if !is_own_coordinate(
            &tetromino_positions[i],
            tetromino_positions,
            &MoveDirection::Down,
        ) {
            if board[(tetromino_positions[i].y + 1) as usize][tetromino_positions[i].x as usize]
                == EMPTY
            {
                allowed_to_drop = true;
            } else if board[(tetromino_positions[i].y + 1) as usize]
                [tetromino_positions[i].x as usize]
                == BLOCK
            {
                return false;
            }
        }
    }
    allowed_to_drop
}

fn is_own_coordinate(
    coordinate: &Coordinate,
    tetromino_positions: &[Coordinate; TETROMINO_SIZE],
    direction: &MoveDirection,
) -> bool {
    let mut is_own_coordinate = false;
    for i in 0..TETROMINO_SIZE {
        match direction {
            MoveDirection::Down => {
                if tetromino_positions[i].x == coordinate.x
                    && tetromino_positions[i].y == coordinate.y + 1
                {
                    is_own_coordinate = true;
                }
            }
            MoveDirection::Left => {
                if tetromino_positions[i].x == coordinate.x - 1
                    && tetromino_positions[i].y == coordinate.y
                {
                    is_own_coordinate = true;
                }
            }
            MoveDirection::Right => {
                if tetromino_positions[i].x == coordinate.x + 1
                    && tetromino_positions[i].y == coordinate.y
                {
                    is_own_coordinate = true;
                }
            }
            _ => {}
        }
    }
    is_own_coordinate
}

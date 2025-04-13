use crate::tetromino::TETROMINO_SIZE;

const EMPTY: char = ' ';
const BLOCK: char = '#';

pub const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 22;

pub enum GameStatus {
    TetrominoBottomCollision,
    TetrominoDropping,
}

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

#[derive(Debug)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

pub fn write_tetromino_to_board(
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
    position_y: isize,
    position_x: isize,
    tetromino: [[char; 4]; 4],
) -> Vec<Coordinate> {
    let mut tetromino_positions: Vec<Coordinate> = Vec::new();

    let mut tetromino_y = 0;
    let mut tetromino_x = 0;

    for i in 0..TETROMINO_SIZE as isize {
        for j in 0..TETROMINO_SIZE as isize {
            if tetromino[tetromino_y][tetromino_x] == BLOCK {
                board[i as usize + position_y as usize][j as usize + position_x as usize] =
                    tetromino[tetromino_y][tetromino_x];
                tetromino_positions.push(Coordinate {
                    x: j + position_x,
                    y: i + position_y,
                });
            }
            if tetromino_x < 4 {
                tetromino_x += 1;
            }
        }
        if tetromino_x >= 4 {
            tetromino_y += 1;
            tetromino_x = 0;
        }
    }
    tetromino_positions
}

pub fn clear_tetromino(
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
    tetromino_positions: &Vec<Coordinate>,
) {
    for position in tetromino_positions {
        board[position.y as usize][position.x as usize] = EMPTY;
    }
}

pub fn display_board(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT], points: usize) {
    for column in board {
        for element in column {
            print!("{} ", element);
        }
        print!("\r\n");
    }
    print!("Points: {}\r\n", points);
}

fn is_coordinate_tetromino_itself(
    check_coordinate: &Coordinate,
    tetromino_positions: &Vec<Coordinate>,
) -> bool {
    for tetromino_position in tetromino_positions {
        if check_coordinate.y == tetromino_position.y && check_coordinate.x == tetromino_position.x
        {
            return true;
        }
    }
    false
}

pub fn tetromino_status(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    tetromino_positions: &Vec<Coordinate>,
) -> GameStatus {
    if check_collision(board, tetromino_positions, &MoveDirection::Down) {
        return GameStatus::TetrominoBottomCollision;
    }

    return GameStatus::TetrominoDropping;
}

pub enum MoveDirection {
    Left,
    Right,
    Down,
}

pub fn move_to_direction(
    tetromino_positions: &Vec<Coordinate>,
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    current_x_position: isize,
    direction: MoveDirection,
) -> isize {
    if !check_collision(board, tetromino_positions, &direction) {
        return match direction {
            MoveDirection::Left => current_x_position - 1,
            MoveDirection::Right => current_x_position + 1,
            _ => current_x_position,
        };
    }
    return current_x_position;
}

pub fn check_collision(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    tetromino_positions: &Vec<Coordinate>,
    direction: &MoveDirection,
) -> bool {
    let mut is_colliding_with_wall = false;
    for position in tetromino_positions {
        let check_coordinate: Coordinate = match direction {
            MoveDirection::Left => Coordinate {
                x: position.x - 1,
                y: position.y,
            },
            MoveDirection::Right => Coordinate {
                x: position.x + 1,
                y: position.y,
            },
            MoveDirection::Down => Coordinate {
                x: position.x,
                y: position.y + 1,
            },
        };
        if board[check_coordinate.y as usize][check_coordinate.x as usize] == BLOCK
            && !is_coordinate_tetromino_itself(&check_coordinate, tetromino_positions)
        {
            is_colliding_with_wall = true;
        }
    }
    is_colliding_with_wall
}

pub fn collapse(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
    let mut full_row_found = false;
    let collapse_columns = read_collapse_count(board);
    for element in collapse_columns {
        if element != 0 {
            overwrite_from_above(board, element);
            full_row_found = true;
        }
    }
    full_row_found
}

fn overwrite_from_above(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT], column: usize) {
    for j in 1..BOARD_WIDTH - 2 {
        board[column][j] = EMPTY;
    }
    for i in (1..BOARD_HEIGHT - 1).rev() {
        for j in 1..BOARD_WIDTH - 2 {
            board[i][j] = board[i - 1][j];
        }
    }
}

pub fn read_collapse_count(
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
) -> [usize; BOARD_HEIGHT - 2] {
    let mut collapse_columns: [usize; BOARD_HEIGHT - 2] = [0; BOARD_HEIGHT - 2];
    let mut index: usize = 0;
    for i in 1..BOARD_HEIGHT - 1 {
        let mut counter = 0;
        for j in 1..BOARD_WIDTH - 1 {
            if board[i][j] == BLOCK {
                counter += 1;
            }
        }

        if counter == BOARD_WIDTH - 2 {
            collapse_columns[index] = i;
            index += 1;
        }
    }
    collapse_columns
}

pub fn game_over(board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
    for i in 1..BOARD_WIDTH - 1 {
        if board[1][i] == BLOCK {
            return true;
        }
    }
    false
}

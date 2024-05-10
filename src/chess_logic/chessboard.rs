/* Defines different piece types and color */

const PAWN: u8 = 0b0001;
const ROOK: u8 = 0b0010;
const BISHOP: u8 = 0b0011;
const KNIGHT: u8 = 0b0100;
const QUEEN: u8 = 0b0101;
const KING: u8 = 0b0111;
const BLACK: u8 = 0b10000;
const WHITE: u8 = 0b1000;
const EMPTY: u8 = 0b0;

const ARR_SIZE: usize = 64;
const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const COL_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const NUM_CHAR: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const VALID_FEN_BOARD: [char; 21] = [
    'p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N', 'Q', 'K', '1', '2', '3', '4', '5', '6', '7',
    '8', '/',
];

const FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct ChessBoard {
    board: [u8; ARR_SIZE],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        return Self { board: [EMPTY; 64] };
    }
    fn make_white(piece: u8) -> u8 {
        return (piece | WHITE);
    }
    fn make_black(piece: u8) -> u8 {
        return (piece | BLACK);
    }
    pub fn set_fen_position(fen: &str) -> Result<(), &'static str> {
        if !is_fen_valid(fen) {
            return Err("NOT VALID FEN");
        }

        let mut fen_index: i32 = (ARR_SIZE as i32) - (ROW_SIZE as i32);
        let mut col_index: i32 = 0;

        // Additional logic for valid FEN goes here...

        // Return Ok(()) to signify success
        Ok(())
    }
}

pub fn is_fen_valid(fen: &str) -> bool {

    let split_fen = split_at_space(fen);

    if (split_fen.len() != 6) {
        return false;
    }

    if !(fen_check_board_validity_optimized(split_fen[0].as_str())  &&
        fen_check_side_to_move(split_fen[1].as_str())               &&
        fen_check_castling_ability(split_fen[2].as_str())           &&
        fen_check_en_passant(split_fen[3].as_str())                 &&
        fen_check_halfmove(split_fen[4].as_str())                   &&
        fen_check_fullmove(split_fen[5].as_str(), split_fen[4].as_str())
    ) {
        return false;
    }

    true
}

fn split_at_space(fen: &str) -> Vec<String> {
    let mut split_fen: Vec<String> = Vec::with_capacity(4 * 90);
    let mut string_buffer: String = String::with_capacity(90);

    for c in fen.chars() {
        if c == ' ' {
            if string_buffer.len() > 0 {
                split_fen.push(string_buffer.clone());
                string_buffer.clear()
            } else {
                continue;
            }
        } else {
            string_buffer.push(c);
        }
    }

    if string_buffer.len() > 0 {
        split_fen.push(string_buffer);
    }

    split_fen
}

fn fen_check_board_validity_optimized(fen: &str) -> bool {
    let mut squares_on_rows: [u32; ROW_SIZE] = [0; ROW_SIZE];
    let mut current_row: usize = ROW_SIZE - 1;

    for c in fen.chars() {
        if c == ' ' {
            if (current_row != 0) && !squares_on_rows.iter().all(|&x| x == ROW_SIZE as u32) {
                return false;
            }
            break;
        };

        let result = VALID_FEN_BOARD.iter().find(|&&x| x == c);
        match result {
            Some(&found_char) => {
                if (found_char == '/') && (current_row > 0) && squares_on_rows[current_row] == 8 {
                    current_row -= 1;
                } else {
                    let squares_to_move: u32 = found_char.to_digit(10).unwrap_or(1);

                    if squares_on_rows[current_row] + squares_to_move <= ROW_SIZE as u32 {
                        squares_on_rows[current_row] += squares_to_move;
                    } else {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
    return true;
}

fn fen_check_side_to_move(side_to_move: &str) -> bool {

    if (side_to_move.len() != 1) {
        return false
    }

    let side_char = side_to_move.chars().next();

    return match side_char {
        Some(c) => {
            if (c == 'w' || c == 'b') { true } else { false }
        }
        None => { false }
    }
}

fn fen_check_castling_ability(castling_string: &str) -> bool {
    let mut castling_ability: [bool; 4] = [false; 4];
    let mut current:usize;

    if fen_check_hyphen(castling_string) {
        return true;
    }

    for c in castling_string.chars() {
        match c {
            'K' => {
                current = 0;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'Q' => {
                current = 1;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'k' => {
                current = 2;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'q' => {
                current = 3;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            _ => {
                return false;
            }
        }
    }
    return true;
}

fn fen_check_en_passant(en_passant: &str) -> bool {
    if fen_check_hyphen(en_passant) {
        return true;
    }

    if !en_passant.len() == 2 {return false}
    else {
        let mut square_iter = en_passant.chars();
        let file_letter = square_iter.next();

        match file_letter {
            Some(c) => {
                if (c == 'a' || c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f' ||
                    c == 'g' || c == 'h'
                ) {
                    let eprank = square_iter.next();

                    match eprank {
                        Some(n) => {
                            let number = n.to_digit(10).unwrap_or(1);
                            if number != 3 || number != 6 {return false}
                        }
                        None => {return false}
                    }
                } else {return false}
            }
            None => {return false}
        }
    }
    true
}

fn fen_check_halfmove(halfmove: &str) -> bool {
    if halfmove.len() > 2 {return false}

    let parsed_halfmove = halfmove.parse::<u32>();

    match parsed_halfmove {
        Ok(number) => {
            if number > 50 {return false}
        }
        Err(E) => {return false}
    }
    true
}

fn fen_check_fullmove(fullmove: &str, halfmove: &str) -> bool {
    let halfmove_result = halfmove.parse::<u32>();
    let halfmove_parsed: u32;
    match halfmove_result {
        Ok(number) => { halfmove_parsed = number}
        Err(E) => {return false}
    }

    let fullmove_result = fullmove.parse::<u32>();
    let fullmove_parsed: u32;
    match fullmove_result {
        Ok(number) => { fullmove_parsed = number}
        Err(E) => {return false}
    }

    return { if fullmove > halfmove {true} else {false} }
}

fn fen_check_hyphen(fen_slice: &str) -> bool {
    if fen_slice.len() == 1 {
        if let Some(letter) = fen_slice.chars().next() {
            return letter == '-';
        }
    }
    false
}
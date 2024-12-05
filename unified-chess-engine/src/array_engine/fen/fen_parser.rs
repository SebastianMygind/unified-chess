use crate::array_engine::{
    Board, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    BOARD_HEIGHT, BOARD_WIDTH, EMPTY_SQUARE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
    WHITE_QUEEN, WHITE_ROOK,
};
use std::str::Chars;
use unified_chess_shared::chess_errors::FenArguments::CastlingAbility;
use unified_chess_shared::shared_types::{Color, Position};

pub struct PositionIterator<'a> {
    fen_chars: Chars<'a>,
    empty_squares: Option<u32>,
}

impl<'a> PositionIterator<'a> {
    pub(crate) fn new(fen: &'a str) -> PositionIterator {
        Self {
            fen_chars: fen.chars(),
            empty_squares: None,
        }
    }

    fn update_empty_squares(&mut self) -> Option<i8> {
        if let Some(remainder) = self.empty_squares {
            self.update(remainder);
            Some(EMPTY_SQUARE)
        } else {
            None
        }
    }

    fn update(&mut self, remainder: u32) {
        match remainder {
            0 => unreachable!("Other checks should go into effect before 0 is reached!"),
            1 => self.empty_squares = None,
            _ => self.empty_squares = Some(remainder - 1),
        }
    }

    fn parse_next_char(&mut self, char: char) -> Option<i8> {
        if let Some(digit) = char.to_digit(10) {
            assert!(self.empty_squares.is_none());

            if digit == 0 {
                unreachable!("FEN input not sanitised for 0 chars in position string")
            } else if digit == 1 {
                self.empty_squares = None;
                return Some(EMPTY_SQUARE);
            }

            self.empty_squares = Some(digit - 1);
            return Some(EMPTY_SQUARE);
        }
        Some(match char {
            'K' => WHITE_KING,
            'Q' => WHITE_QUEEN,
            'R' => WHITE_ROOK,
            'B' => WHITE_BISHOP,
            'N' => WHITE_KNIGHT,
            'P' => WHITE_PAWN,

            'k' => BLACK_KING,
            'q' => BLACK_QUEEN,
            'r' => BLACK_ROOK,
            'b' => BLACK_BISHOP,
            'n' => BLACK_KNIGHT,
            'p' => BLACK_PAWN,
            _ => return None,
        })
    }
}

impl Iterator for PositionIterator<'_> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(empty_square) = self.update_empty_squares() {
            return Some(empty_square);
        }

        let mut char = self.fen_chars.next()?;

        if char == '/' {
            char = self
                .fen_chars
                .next()
                .expect("should be validated in fen_validator");
        }

        self.parse_next_char(char)
    }
}

pub fn parse_position(position: &str) -> Option<Board> {
    let mut board: Board = [EMPTY_SQUARE; BOARD_WIDTH * BOARD_HEIGHT];

    let fen_iterator = PositionIterator::new(position);

    let mut row_1 = 8;
    let mut col_1 = 0;

    for square in fen_iterator {
        board[((row_1 - 1) * 8) + col_1] = square;

        col_1 += 1;

        if col_1 == 8 {
            row_1 -= 1;
            col_1 = 0;
        }
    }
    if !(row_1 == 0 && col_1 == 0) {
        return None;
    }
    Some(board)
}

pub fn parse_side_to_move(fen_part: &str) -> Option<Color> {
    let side_to_move: Color;

    let side_to_move_char = fen_part.chars().next()?;

    match side_to_move_char {
        'w' => side_to_move = Color::White,
        'b' => side_to_move = Color::Black,
        _ => return None,
    }

    Some(side_to_move)
}

pub fn parse_castling_abilities(fen_part: &str) -> Option<[bool; 4]> {
    let mut castling_ability = [false; 4];

    for char in fen_part.chars() {
        match char {
            'K' => castling_ability[0] = true,
            'Q' => castling_ability[1] = true,
            'k' => castling_ability[2] = true,
            'q' => castling_ability[3] = true,
            '-' => break,
            _ => return None,
        }
    }
    Some(castling_ability)
}

pub fn parse_epawn(fen_part: &str) -> Option<Option<Position>> {
    let mut position: Position = Position { x: 0, y: 0 };

    let mut chars = fen_part.chars();

    let file = chars.next()?;
    if file == '-' {
        return Some(None);
    }

    position.x = parse_file(file)?;

    let rank = chars.next()?;
    position.y = parse_rank(rank)?;

    Some(Some(position))
}

fn parse_file(char: char) -> Option<usize> {
    Some(match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None,
    })
}

fn parse_rank(char: char) -> Option<usize> {
    Some(match char {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => return None,
    })
}

pub fn parse_string_to_num(fen_part: &str) -> Option<u32> {
    Some(fen_part.parse().expect("Should be validated"))
}

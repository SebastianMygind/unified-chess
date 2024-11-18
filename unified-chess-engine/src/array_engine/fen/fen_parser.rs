use crate::array_engine::{
    Board, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    BOARD_HEIGHT, BOARD_WIDTH, EMPTY_SQUARE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
    WHITE_QUEEN, WHITE_ROOK,
};
use std::io::empty;
use std::str::Chars;
use unified_chess_shared::shared_types::Position;

struct PositionIterator<'a> {
    fen_chars: Chars<'a>,
    empty_remainder: Option<u32>,
}

impl PositionIterator<'_> {
    fn new(fen: &str) -> PositionIterator {
        Self {
            fen_chars: fen.chars(),
            empty_remainder: None,
        }
    }

    fn update_remainder(&mut self) -> Option<i8> {
        if let Some(remainder) = self.empty_remainder {
            self.update(remainder);
            Some(EMPTY_SQUARE)
        } else {
            None
        }
    }

    fn update(&mut self, remainder: u32) {
        if remainder == 1 {
            self.empty_remainder = None;
        }
        self.empty_remainder = Some(remainder - 1);
    }

    fn parse_next_char(&mut self, char: char) -> Option<i8> {
        if let Some(digit) = char.to_digit(10) {
            self.empty_remainder = Some(digit - 1);
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
        if let Some(piece) = self.update_remainder() {
            return Some(piece);
        }
        self.parse_next_char(self.fen_chars.next()?)
    }
}

pub fn parse_position(position: &str) -> Option<Board> {
    let mut board: Board = [EMPTY_SQUARE; BOARD_WIDTH * BOARD_HEIGHT];

    let mut fen_iterator = PositionIterator::new(position);

    let mut row = 7;
    let mut col = 0;

    for position in fen_iterator {
        board[col + (8 * row)] = position;
        if col == 7 {
            col = 0;
            row -= 1;
        }
    }

    Some(board)
}

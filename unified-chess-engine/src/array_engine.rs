mod fen;
mod pieces;

use unified_chess_shared::shared_types::SharedState;
use unified_chess_shared::shared_types::{Color, Move, Piece, Position};

const WHITE_KING: i8 = 1;
const WHITE_QUEEN: i8 = 2;
const WHITE_ROOK: i8 = 3;
const WHITE_BISHOP: i8 = 4;
const WHITE_KNIGHT: i8 = 5;
const WHITE_PAWN: i8 = 6;

const EMPTY_SQUARE: i8 = 0;

const BLACK_KING: i8 = -1;
const BLACK_QUEEN: i8 = -2;
const BLACK_ROOK: i8 = -3;
const BLACK_BISHOP: i8 = -4;
const BLACK_KNIGHT: i8 = -5;
const BLACK_PAWN: i8 = -6;

const BOARD_HEIGHT: usize = 8;
const BOARD_WIDTH: usize = 8;

const MAILBOX: [i8; (BOARD_WIDTH + 2) * (BOARD_HEIGHT + 4)] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, 57, 58,
    59, 60, 61, 62, 63, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 40, 41, 42, 43, 44, 45, 46,
    47, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 16,
    17, 18, 19, 20, 21, 22, 23, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 0, 1, 2, 3, 4, 5, 6,
    7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

type Board = [i8; BOARD_HEIGHT * BOARD_WIDTH];

pub struct ChessState {
    board: Board,
    side_to_move: Color,
    castling_ability: [bool; 4],
    en_passant_target_square: Option<Position>,
    half_move_clock: u32,
    full_move_counter: u32,
}

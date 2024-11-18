mod fen;
mod pieces;

use std::rc::Rc;
use unified_chess_shared::shared_types::ChessState as SharedState;
use unified_chess_shared::shared_types::{Color, Move, Piece, Position};

type Square = Option<Box<dyn ChessPiece>>;

struct Board {
    board: Vec<Vec<Square>>,
}

struct ChessState {
    board: Board,
    side_to_move: Color,
    castling_ability: [bool; 4],
    en_passant_target_square: Option<Position>,
    half_move_clock: u32,
    full_move_counter: u32,
}


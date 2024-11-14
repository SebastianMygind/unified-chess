mod fen;
mod pieces;

use std::rc::Rc;
use unified_chess_shared::shared_types::ChessState as SharedState;
use unified_chess_shared::shared_types::{Color, Move, Piece, Position};

type Board<'a> = [[Option<&'a Rc<dyn ChessPiece>>; 8]; 8];

struct ChessState<'a> {
    board: Board<'a>,
    side_to_move: Color,
    castling_ability: [bool; 4],
    en_passant_target_square: Option<Position>,
    half_move_clock: u32,
    full_move_counter: u32,
}

impl ChessState<'_> {
    fn new() -> Self {
        const EMPTY_PIECE: Option<&Rc<dyn ChessPiece>> = None;

        Self {
            board: [[EMPTY_PIECE; 8]; 8],
            side_to_move: Color::White,
            castling_ability: [true; 4],
            en_passant_target_square: None,
            half_move_clock: 0,
            full_move_counter: 0,
        }
    }
}

trait ChessPiece {
    fn get_moves(&self, chess_board: ChessState) -> Vec<Move>;

    fn get_type(&self) -> Piece;
}

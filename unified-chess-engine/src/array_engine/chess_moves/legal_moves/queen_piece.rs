use crate::array_engine::chess_moves::legal_moves::generic_piece::get_multi_step_moves;
use crate::array_engine::chess_moves::piece_logic::KING_AND_QUEEN_DIRECTION;
use crate::array_engine::{ChessBoard, Color, Move, PieceType, Position};

pub fn get_queen_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    get_multi_step_moves(
        chess_board,
        piece_position,
        PieceType::Queen,
        friendly_color,
        KING_AND_QUEEN_DIRECTION.as_slice(),
    )
}

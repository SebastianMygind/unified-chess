use crate::array_engine::{ChessPiece, ChessState};
use unified_chess_shared::shared_types::{Color, Move, Piece, PieceType};

struct Queen {
    color: Color,
}

impl ChessPiece for Queen {
    fn get_moves(&self, chess_board: ChessState) -> Vec<Move> {
        todo!()
    }

    fn get_type(&self) -> Piece {
        Piece::new(PieceType::Queen, self.color)
    }
}

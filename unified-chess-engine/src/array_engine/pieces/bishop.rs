use crate::array_engine::{ChessPiece, ChessState};
use unified_chess_shared::shared_types::{Move, Piece};

struct Bishop;

impl ChessPiece for Bishop {
    fn get_moves(&self, chess_board: ChessState) -> Vec<Move> {
        todo!()
    }

    fn get_type(&self) -> Piece {
        todo!()
    }
}

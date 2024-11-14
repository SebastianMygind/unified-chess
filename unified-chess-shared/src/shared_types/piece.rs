use crate::shared_types::{Color, Piece, PieceType};

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: Color) -> Self {
        Self {
            piece_type,
            piece_color,
        }
    }
}

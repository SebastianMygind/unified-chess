use crate::shared_types::{Color, Piece, PieceType};

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: Color) -> Option<Self> {
        Some(Self {
            piece_type,
            piece_color,
        })
    }
}

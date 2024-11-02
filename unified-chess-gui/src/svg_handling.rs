use std::env;

use crate::ColoredPieces;

pub struct SvgPieces {
    pub white_king: &'static [u8],
    pub white_queen: &'static [u8],
    pub white_rook: &'static [u8],
    pub white_bishop: &'static [u8],
    pub white_knight: &'static [u8],
    pub white_pawn: &'static [u8],
    pub black_king: &'static [u8],
    pub black_queen: &'static [u8],
    pub black_rook: &'static [u8],
    pub black_bishop: &'static [u8],
    pub black_knight: &'static [u8],
    pub black_pawn: &'static [u8],
}

impl Default for SvgPieces {
    fn default() -> Self {
        let white_king = include_bytes!("../../pieces/cburnett/wK.svg");
        let white_queen = include_bytes!("../../pieces/cburnett/wQ.svg");
        let white_rook = include_bytes!("../../pieces/cburnett/wR.svg");
        let white_bishop = include_bytes!("../../pieces/cburnett/wB.svg");
        let white_knight = include_bytes!("../../pieces/cburnett/wN.svg");
        let white_pawn = include_bytes!("../../pieces/cburnett/wP.svg");
        let black_king = include_bytes!("../../pieces/cburnett/bK.svg");
        let black_queen = include_bytes!("../../pieces/cburnett/bQ.svg");
        let black_rook = include_bytes!("../../pieces/cburnett/bR.svg");
        let black_bishop = include_bytes!("../../pieces/cburnett/bB.svg");
        let black_knight = include_bytes!("../../pieces/cburnett/bN.svg");
        let black_pawn = include_bytes!("../../pieces/cburnett/bP.svg");
        SvgPieces {
            white_king,
            white_queen,
            white_rook,
            white_bishop,
            white_knight,
            white_pawn,
            black_king,
            black_queen,
            black_rook,
            black_bishop,
            black_knight,
            black_pawn,
        }
    }
}

impl SvgPieces {
    fn get_piece_bytes(&self, piece_to_get: ColoredPieces) -> &[u8] {
        return match piece_to_get {
            ColoredPieces::WKing => self.white_king,
            ColoredPieces::WQueen => self.white_queen,
            ColoredPieces::WRook => self.white_rook,
            ColoredPieces::WBishop => self.white_bishop,
            ColoredPieces::WKnight => self.white_knight,
            ColoredPieces::WPawn => self.white_pawn,

            ColoredPieces::BKing => self.black_king,
            ColoredPieces::BQueen => self.black_queen,
            ColoredPieces::BRook => self.black_rook,
            ColoredPieces::BBishop => self.black_bishop,
            ColoredPieces::BKnight => self.black_knight,
            ColoredPieces::BPawn => self.black_pawn,
        };
    }
}

use crate::array_engine::{
    Board, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    BOARD_HEIGHT, BOARD_WIDTH, EMPTY_SQUARE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN,
    WHITE_QUEEN, WHITE_ROOK,
};
use std::str::Chars;
use unified_chess_shared::shared_types::Color;

mod test {
    use super::*;
    use crate::array_engine::fen::fen_parser::*;

    #[test]
    fn test_fen_iterator1() {
        let pos_iter = PositionIterator::new("r2/1NQK");

        let mut parsed_pieces = Vec::with_capacity(7);

        for piece in pos_iter {
            parsed_pieces.push(piece);
        }

        let expected_pieces = vec![
            BLACK_ROOK,
            EMPTY_SQUARE,
            EMPTY_SQUARE,
            EMPTY_SQUARE,
            WHITE_KNIGHT,
            WHITE_QUEEN,
            WHITE_KING,
        ];

        assert_eq!(parsed_pieces, expected_pieces);
    }

    #[test]
    fn test_fen_iterator2() {
        let mut pos_iter = PositionIterator::new("rRRr/4P");

        let mut vec: Vec<i8> = Vec::with_capacity(9);

        let mut current_len = 0;

        for i in 1..=9 {
            vec.push(pos_iter.next().unwrap());
            current_len += 1;
        }
        assert_eq!(current_len, 9);
    }

    #[test]
    fn test_fen_iterator3() {
        assert_eq!(PositionIterator::new("r/4P").count(), 6);
    }

    #[test]
    fn test_parse_position1() {
        assert_eq!(parse_position(""), None);
    }

    #[test]
    fn test_parse_position2() {
        assert_eq!(
            parse_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Some([
                WHITE_ROOK,
                WHITE_KNIGHT,
                WHITE_BISHOP,
                WHITE_QUEEN,
                WHITE_KING,
                WHITE_BISHOP,
                WHITE_KNIGHT,
                WHITE_ROOK,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                WHITE_PAWN,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_PAWN,
                BLACK_ROOK,
                BLACK_KNIGHT,
                BLACK_BISHOP,
                BLACK_QUEEN,
                BLACK_KING,
                BLACK_BISHOP,
                BLACK_KNIGHT,
                BLACK_ROOK,
            ])
        );
    }

    #[test]
    fn test_parse_position3() {
        assert_eq!(
            parse_position("8/8/8/8/8/8/8/8"),
            Some([EMPTY_SQUARE; BOARD_WIDTH * BOARD_HEIGHT])
        );
    }
}

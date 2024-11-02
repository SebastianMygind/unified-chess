pub mod shared_types {
    pub enum MoveData {
        CastlingMove,
        EnPassant,
        Check,
        CheckMate,
        StaleMate,
        FiftyMoveRule,
    }

    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    pub struct Move {
        from: Position,
        to: Position,
        data: MoveData,
    }

    pub struct RatedMove {
        chess_move: Move,
        rating: i32,
    }

    pub enum PieceType {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    pub enum Color {
        White,
        Black,
    }

    pub struct Piece {
        piece_type: PieceType,
        piece_color: Color,
    }

    const BOARD_WIDTH: usize = 8;
    const BOARD_HEIGHT: usize = 8;
    pub type Board = [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH];

    pub struct ChessState {
        board: Board,
        side_to_move: Color,
        castling_ability: [bool; 4], // WKingside, WQueenside, BKingside, BQueenside
        en_passant_target_square: Option<Position>,
        half_move_clock: u32,
        full_move_counter: u32,
    }
}

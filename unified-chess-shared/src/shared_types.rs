pub mod piece;

pub enum MoveData {
    CastlingMove,
    EnPassant,
    Check,
    CheckMate,
    StaleMate,
    FiftyMoveRule,
}

#[derive(Copy, Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Debug, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: Color,
}

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
pub type Board = [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH];

pub struct SharedState {
    pub board: Board,
    pub side_to_move: Color,
    pub castling_ability: [bool; 4], // WKingside, WQueenside, BKingside, BQueenside
    pub en_passant_target_square: Option<Position>,
    pub half_move_clock: u32,
    pub full_move_counter: u32,
}

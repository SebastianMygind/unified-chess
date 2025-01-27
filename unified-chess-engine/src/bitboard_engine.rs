use unified_chess_shared::shared_types::SharedState;
use unified_chess_shared::shared_types::{Color, Position};

struct Board {
    white_king: i64,
    white_queen: i64,
    white_rook: i64,
    white_bishop: i64,
    white_knight: i64,
    white_pawn: i64,

    black_king: i64,
    black_queen: i64,
    black_rook: i64,
    black_bishop: i64,
    black_knight: i64,
    black_pawn: i64,
}

struct ChessState {
    board: Board,
    side_to_move: Color,
    castling_ability: [bool; 4],
    en_passant_target_square: Option<Position>,
    half_move_clock: u32,
    full_move_counter: u32,
}

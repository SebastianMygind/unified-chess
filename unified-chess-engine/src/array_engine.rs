use crate::fen::{
    is_fen_valid, parse_fen_castling_ability, parse_fen_epawn, parse_fen_full_move_counter,
    parse_fen_half_move_clock, parse_fen_piece_placement, parse_fen_side_to_move, split_at_space,
};

/* Module that allows printing a chessboard to the CLI */
mod chess_display;
pub mod chess_errors;
pub mod chess_moves;
pub(crate) mod perft;

use crate::fen::FEN_START_POSITION;
use chess_errors::InvalidFen;

pub const ROW_SIZE: usize = 8;
pub const COL_SIZE: usize = 8;

pub(crate) type Board = [[Square; COL_SIZE]; ROW_SIZE];
pub type Position = (usize, usize);
pub(crate) type Square = Option<Piece>;

/** Defines different chess piece types. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

/** Defines the colors chess pieces can have. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/** Defines the struct that describes a complete chess piece. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MoveResult {
    Normal,
    Check,
    Checkmate,
    Stalemate,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct MoveMetaData {
    piece_to_move: PieceType,
    piece_to_capture: Option<PieceType>,
    pub promotion_piece: Option<PieceType>,
    is_castling_move: bool,
    generates_en_passant: bool,
    is_en_passant_move: bool,
    //move_result: MoveResult,
}

/* Chessboard specific implementations */
#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    board: Board,
    white_is_side_to_move: bool,
    castling_ability: [bool; 4], // WKingside, WQueenside, BKingside, BQueenside
    en_passant_target_square: Option<Position>,
    half_move_clock: u64,
    full_move_counter: u64,
    //is_checked: bool,
    //is_checkmate: bool,
    //is_stalemate: bool,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Move {
    pub start_pos: Position,
    pub end_pos: Position,
    pub meta_data: MoveMetaData,
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        let new_board: ChessBoard = ChessBoard::new_from_fen(FEN_START_POSITION)
            .expect("ERROR: FEN starting position does not parse correctly!");

        new_board
    }

    pub fn new_from_fen(fen: &str) -> Result<ChessBoard, InvalidFen> {
        let mut new_board: ChessBoard = ChessBoard {
            board: [[None; COL_SIZE]; ROW_SIZE],
            white_is_side_to_move: true,
            castling_ability: [true; 4],
            en_passant_target_square: None,
            half_move_clock: 0,
            full_move_counter: 0,
            //is_checked: false,
            //is_checkmate: false,
            //is_stalemate: false,
        };

        match new_board.set_fen_position_arr(fen) {
            Ok(_) => Ok(new_board),
            Err(e) => Err(e),
        }
    }

    // Implements FEN functionality
    pub fn set_fen_position_arr(&mut self, fen: &str) -> Result<(), InvalidFen> {
        if !is_fen_valid(fen) {
            return Err(InvalidFen);
        }
        let split_fen = split_at_space(fen);

        /* Piece placement */
        let parsed_board = parse_fen_piece_placement(&split_fen[0].as_str());

        self.board = parsed_board;

        /* Side to move */
        let is_white_move = parse_fen_side_to_move(&split_fen[1].as_str());

        self.white_is_side_to_move = is_white_move;

        /* Castling ability*/
        let castling_ability = parse_fen_castling_ability(&split_fen[2].as_str());

        self.castling_ability = castling_ability;

        /* En Passant */
        let en_passant = parse_fen_epawn(&split_fen[3].as_str());

        self.en_passant_target_square = en_passant;

        /* Half move clock */
        let half_moves = parse_fen_half_move_clock(&split_fen[4].as_str());

        self.half_move_clock = half_moves;

        /* Full move counter */
        let full_moves = parse_fen_full_move_counter(&split_fen[5].as_str());

        self.full_move_counter = full_moves;

        Ok(())
    }
}

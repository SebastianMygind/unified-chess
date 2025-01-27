mod fen_parser;
mod fen_parser_tests;

use crate::array_engine::fen::fen_parser::{
    parse_castling_abilities, parse_epawn, parse_position, parse_side_to_move, parse_string_to_num,
};
use crate::array_engine::ChessState;
use crate::array_engine::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, EMPTY_SQUARE,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use unified_chess_shared::chess_errors::{FenArguments, FenError, FenErrorKind, MoveError};
use unified_chess_shared::shared_types::Color::{Black, White};
use unified_chess_shared::shared_types::{
    Board, Color, Move, Piece, PieceType, RatedMove, SharedState,
};
use unified_chess_shared::state::{ChessEngine, FenState, FenType};

impl ChessEngine for ChessState {
    fn get_legal_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn get_best_moves(
        &self,
        depth: usize,
        previous_depth_best_moves: Option<Vec<RatedMove>>,
    ) -> Vec<RatedMove> {
        todo!()
    }

    fn make_move(&mut self, mv: Move) -> Result<(), MoveError> {
        todo!()
    }

    fn set_state(&mut self, fen_state: &str) -> Result<(), FenError> {
        todo!()
    }

    fn fen_to_state(&self, fen: &str) -> Result<Box<Self>, FenError> {
        let fen_type = Self::is_fen_valid(fen)?;

        let mut fen_state: FenState = FenState::new(fen);

        let position = parse_fen_part(&mut fen_state, parse_position, FenArguments::Position)?;

        let side_to_move =
            parse_fen_part(&mut fen_state, parse_side_to_move, FenArguments::SideToMove)?;

        let castling_ability = parse_fen_part(
            &mut fen_state,
            parse_castling_abilities,
            FenArguments::CastlingAbility,
        )?;

        let e_pawn = parse_fen_part(
            &mut fen_state,
            parse_epawn,
            FenArguments::EnPassantTargetSquare,
        )?;

        if fen_type == FenType::NoCounter {
            return Ok(Box::new(ChessState {
                board: position,
                side_to_move,
                castling_ability,
                en_passant_target_square: e_pawn,
                half_move_clock: 0,
                full_move_counter: 0,
            }));
        }

        let halv_moves = parse_fen_part(
            &mut fen_state,
            parse_string_to_num,
            FenArguments::HalfMoveClock,
        )?;
        let full_moves = parse_fen_part(
            &mut fen_state,
            parse_string_to_num,
            FenArguments::FullMoveCounter,
        )?;

        Ok(Box::new(ChessState {
            board: position,
            side_to_move,
            castling_ability,
            en_passant_target_square: e_pawn,
            half_move_clock: halv_moves,
            full_move_counter: full_moves,
        }))
    }

    fn state_to_fen(&self) -> String {
        todo!()
    }

    fn state_to_shared_state(&self) -> SharedState {
        let mut shared_board: Board = [[None; 8]; 8];

        for (index, piece) in self.board.iter().enumerate() {
            match *piece {
                EMPTY_SQUARE => continue,

                WHITE_KING => {
                    set_piece(&mut shared_board, Piece::new(PieceType::King, White), index)
                }
                WHITE_QUEEN => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Queen, White),
                    index,
                ),
                WHITE_ROOK => {
                    set_piece(&mut shared_board, Piece::new(PieceType::Rook, White), index)
                }
                WHITE_BISHOP => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Bishop, White),
                    index,
                ),
                WHITE_KNIGHT => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Knight, White),
                    index,
                ),
                WHITE_PAWN => {
                    set_piece(&mut shared_board, Piece::new(PieceType::Pawn, White), index)
                }

                BLACK_KING => {
                    set_piece(&mut shared_board, Piece::new(PieceType::King, Black), index)
                }
                BLACK_QUEEN => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Queen, Black),
                    index,
                ),
                BLACK_ROOK => {
                    set_piece(&mut shared_board, Piece::new(PieceType::Rook, Black), index)
                }
                BLACK_BISHOP => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Bishop, Black),
                    index,
                ),
                BLACK_KNIGHT => set_piece(
                    &mut shared_board,
                    Piece::new(PieceType::Knight, Black),
                    index,
                ),
                BLACK_PAWN => {
                    set_piece(&mut shared_board, Piece::new(PieceType::Pawn, Black), index)
                }
                _ => unreachable!(),
            }
        }

        SharedState {
            board: shared_board,
            side_to_move: self.side_to_move,
            castling_ability: self.castling_ability,
            en_passant_target_square: self.en_passant_target_square,
            half_move_clock: self.half_move_clock,
            full_move_counter: self.full_move_counter,
        }
    }

    fn shared_state_to_state(shared_state: &SharedState) -> Self {
        todo!()
    }
}

fn parse_fen_part<F, T>(
    fen_state: &mut FenState,
    parser: F,
    argument: FenArguments,
) -> Result<T, FenError>
where
    F: Fn(&str) -> Option<T>,
{
    let parsed_part: T = match parser(
        fen_state
            .fen_part
            .expect("Validated fen should have fen_part!"),
    ) {
        Some(parsed_part) => parsed_part,
        None => {
            return Err(FenError::new(
                FenErrorKind::ParserError(argument),
                fen_state.fen,
            ))
        }
    };

    fen_state.update();

    Ok(parsed_part)
}

fn set_piece(board: &mut Board, piece: Option<Piece>, position: usize) {
    let x = position / 8;
    let y = position % 8;

    board[x][y] = piece;
}

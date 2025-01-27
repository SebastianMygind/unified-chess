pub mod fen_strings;
mod fen_validation;

use crate::chess_errors::{FenArguments, FenError, FenErrorKind, MoveError};
use crate::shared_types::{Move, RatedMove, SharedState};
use crate::state::fen_validation::{
    is_castling_valid, is_en_passant_valid, is_half_move_valid, is_move_counter_valid,
    is_position_valid, is_side_to_move_valid,
};
use std::str::Split;

#[derive(Debug, PartialEq)]
pub enum FenType {
    Full,
    NoCounter,
}

#[derive(Debug)]
pub struct FenState<'a> {
    pub fen: &'a str,
    fen_split: Split<'a, char>,
    pub fen_part: Option<&'a str>,
}

impl<'a> FenState<'a> {
    pub fn new(fen: &'a str) -> Self {
        let mut fen_split = fen.split(' ');

        let fen_part = fen_split.next();

        let mut fen_state = FenState {
            fen,
            fen_split,
            fen_part,
        };

        while fen_state.part_str_is_empty_and_some() {
            fen_state.fen_part = fen_state.fen_split.next()
        }

        fen_state
    }

    pub fn update(&mut self) {
        self.fen_part = self.fen_split.next();

        // Allows for incorrect spacing between fen parts
        while self.part_str_is_empty_and_some() {
            self.fen_part = self.fen_split.next()
        }
    }

    pub fn part_str_is_empty_and_some(&self) -> bool {
        if let Some(string) = self.fen_part {
            return string.is_empty();
        };
        false
    }
}

pub trait ChessEngine {
    fn get_legal_moves(&self) -> Vec<Move>;

    fn get_best_moves(
        &self,
        depth: usize,
        previous_depth_best_moves: Option<Vec<RatedMove>>,
    ) -> Vec<RatedMove>;

    fn make_move(&mut self, mv: Move) -> Result<(), MoveError>;

    fn get_state(&self) -> SharedState {
        self.state_to_shared_state()
    }

    fn set_state(&mut self, fen_state: &str) -> Result<(), FenError>;

    fn fen_to_state(&self, fen: &str) -> Result<Box<Self>, FenError>;

    fn state_to_fen(&self) -> String;

    fn state_to_shared_state(&self) -> SharedState;

    fn shared_state_to_state(shared_state: &SharedState) -> Self;

    fn is_fen_valid(fen: &str) -> Result<FenType, FenError> {
        let mut fen_state = FenState::new(fen);

        validate_fen_part(&fen_state, FenArguments::Position, is_position_valid)?;
        fen_state.update();

        validate_fen_part(&fen_state, FenArguments::SideToMove, is_side_to_move_valid)?;
        fen_state.update();

        validate_fen_part(&fen_state, FenArguments::CastlingAbility, is_castling_valid)?;
        fen_state.update();

        validate_fen_part(
            &fen_state,
            FenArguments::EnPassantTargetSquare,
            is_en_passant_valid,
        )?;
        fen_state.update();

        match validate_fen_part(&fen_state, FenArguments::HalfMoveClock, is_half_move_valid) {
            Ok(_) => {}
            Err(e) => {
                return match e {
                    FenError {
                        kind: FenErrorKind::MissingArgument(FenArguments::HalfMoveClock),
                        given_string: fen,
                    } => Ok(FenType::NoCounter),
                    _ => return Err(e),
                }
            }
        }
        fen_state.update();

        _ = validate_fen_part(
            &fen_state,
            FenArguments::FullMoveCounter,
            is_move_counter_valid,
        )?;
        fen_state.update();

        if fen_state.fen_part.is_some() {
            return Err(FenError {
                kind: FenErrorKind::TooManyArguments,
                given_string: fen.to_string(),
            });
        }

        Ok(FenType::Full)
    }
}

fn validate_fen_part<F>(
    fen_state: &FenState,
    fen_argument: FenArguments,
    validator: F,
) -> Result<(), FenError>
where
    F: Fn(&str) -> bool,
{
    if let Some(part_str) = fen_state.fen_part {
        if !validator(part_str) {
            Err(FenError::new(
                FenErrorKind::InvalidArgument(fen_argument),
                part_str,
            ))
        } else {
            Ok(())
        }
    } else {
        Err(FenError::missing_argument(fen_argument, fen_state.fen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_errors::FenErrorKind::{InvalidArgument, MissingArgument};
    use crate::state::fen_strings::{KIWI_PETE, LEGAL_POS1, LEGAL_POS2, START_POS};

    struct TestFenConverter;

    impl ChessEngine for TestFenConverter {
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
            todo!()
        }

        fn state_to_fen(&self) -> String {
            todo!()
        }

        fn state_to_shared_state(&self) -> SharedState {
            todo!()
        }

        fn shared_state_to_state(shared_state: &SharedState) -> Self {
            todo!()
        }
    }

    #[test]
    fn fen_splitter_test1() {
        let mut fen_state = FenState::new(START_POS);

        fen_state.update();

        assert_eq!(fen_state.fen_part, Some("w"));
    }

    #[test]
    fn fen_splitter_test2() {
        let mut fen_state = FenState::new(START_POS);

        fen_state.update();
        fen_state.update();

        assert_eq!(fen_state.fen_part, Some("KQkq"));
    }

    #[test]
    fn fen_validator1() {
        assert_eq!(TestFenConverter::is_fen_valid(START_POS), Ok(FenType::Full));
    }
    #[test]
    fn fen_validator2() {
        assert_eq!(
            TestFenConverter::is_fen_valid(LEGAL_POS1),
            Ok(FenType::Full)
        );
    }
    #[test]
    fn fen_validator3() {
        assert_eq!(
            TestFenConverter::is_fen_valid(LEGAL_POS2),
            Ok(FenType::Full)
        );
    }
    #[test]
    fn fen_validator4() {
        assert_eq!(
            TestFenConverter::is_fen_valid(KIWI_PETE),
            Ok(FenType::NoCounter)
        );
    }
    #[test]
    fn fen_validator5() {
        assert_eq!(
            TestFenConverter::is_fen_valid(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPhPPPP/RNBQKBNR w KQkq - 0 1"
            ),
            Err(FenError::new(
                FenErrorKind::InvalidArgument(FenArguments::Position),
                "rnbqkbnr/pppppppp/8/8/8/8/PPPhPPPP/RNBQKBNR"
            ))
        )
    }

    #[test]
    fn fen_validator6() {
        assert_eq!(
            TestFenConverter::is_fen_valid(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR  KQkq - 0 1"
            ),
            Err(FenError::new(
                InvalidArgument(FenArguments::SideToMove),
                "KQkq"
            ))
        )
    }

    #[test]
    fn fen_validator7() {
        assert_eq!(
            TestFenConverter::is_fen_valid("    "),
            Err(FenError::new(
                FenErrorKind::MissingArgument(FenArguments::Position),
                "    "
            ))
        )
    }
}

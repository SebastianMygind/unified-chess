mod fen_validation;

use crate::chess_errors::{FenArguments, FenError, FenErrorKind};
use crate::shared_types::ChessState;
use crate::state::fen_validation::is_position_valid;

pub trait FenConversion {
    fn fen_to_state(fen: &str) -> Result<Box<Self>, FenError>;

    fn state_to_fen(&self) -> String;

    fn state_to_shared_state(&self) -> ChessState;

    fn shared_state_to_state(shared_state: &ChessState) -> Self;

    fn is_fen_valid(fen: &str) -> Result<(), FenError> {
        let mut split_fen = fen.split(' ');

        if let Some(position_str) = split_fen.next() {
            if !is_position_valid(position_str) {
                return Err(FenError::new(
                    FenErrorKind::InvalidArgument(FenArguments::Position),
                    position_str,
                ));
            }
        } else {
            return Err(FenError::missing_argument(FenArguments::Position, fen));
        }

        Ok(())
    }
}

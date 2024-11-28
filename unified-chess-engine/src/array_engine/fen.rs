mod fen_parser;
mod fen_parser_tests;

use crate::array_engine::fen::fen_parser::{parse_position, parse_side_to_move};
use crate::array_engine::ChessState;
use unified_chess_shared::chess_errors::{FenArguments, FenError, FenErrorKind};
use unified_chess_shared::shared_types::{ChessState as SharedState, Color};
use unified_chess_shared::state::{FenConversion, FenState};

impl FenConversion for ChessState {
    fn fen_to_state(fen: &str) -> Result<Box<Self>, FenError> {
        let fen_type = Self::is_fen_valid(fen)?;

        let mut fen_state: FenState = FenState::new(fen);

        let parsed_position =
            parse_fen_part(&mut fen_state, parse_position, FenArguments::Position)?;
        
        let parsed_side_to_move = 
            parse_fen_part(&mut fen_state, parse_side_to_move, FenArguments::SideToMove)?;
        
        
        
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

use crate::array_engine::ChessState;
use unified_chess_shared::chess_errors::{FenArguments, FenError, FenErrorKind};
use unified_chess_shared::shared_types::{ChessState as SharedState, Color};
use unified_chess_shared::state::FenConversion;

impl FenConversion for ChessState<'_> {
    fn fen_to_state(fen: &str) -> Result<Box<Self>, FenError> {
        _ = Self::is_fen_valid(fen)?;

        let split_fen = fen.split(' ');

        let state: Self = ChessState {
            board: [[None; 8]; 8],
            side_to_move: Color::White,
            castling_ability: [true; 4],
            en_passant_target_square: None,
            half_move_clock: 0,
            full_move_counter: 0,
        };

        Ok(Box::new(state))
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

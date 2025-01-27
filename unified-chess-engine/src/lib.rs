use unified_chess_shared::shared_types::{Move, RatedMove, SharedState};

pub mod array_engine;
mod bitboard_engine;
mod state_manager;

trait ChessEngine {
    fn set_state(&mut self, state: SharedState);
    fn get_state(&self) -> SharedState;
    fn get_legal_moves(&self) -> Vec<Move>;
    fn get_best_moves(&self) -> Vec<RatedMove>;
}

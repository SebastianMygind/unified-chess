use crate::state_manager::EngineTypes::ArrayEngine;
use crate::ChessEngine;
use unified_chess_shared::shared_types::SharedState;

pub enum EngineTypes {
    ArrayEngine,
    BitBoardEngine,
}

pub struct StateManager;

impl StateManager {
    pub fn setup_engine(engine: EngineTypes, state: SharedState) -> Box<dyn ChessEngine> {
        let engine = match engine {
            EngineTypes::ArrayEngine => {}
            EngineTypes::BitBoardEngine => {
                todo!()
            }
        };

        todo!()
    }
}

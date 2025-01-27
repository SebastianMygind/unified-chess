use unified_chess_shared::shared_types::SharedState;

const MENU_OPTIONS: &str = "\
Start new game  (N/n)\n\
Continue a game (C/c)\n\
Exit the game   (E/e,Q/q)";

enum Modes {
    NewGame,
    Exit,
    Continue,
}

pub struct CommandLineInterface {}

impl CommandLineInterface {
    pub fn run() {
        println!("Welcome to unified-chess!");
        println!("{MENU_OPTIONS}");

        let mut ChessState: SharedState = SharedState::new();
    }
}

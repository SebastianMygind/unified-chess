use std::env;
use unified_chess_gui;
use unified_chess_gui::ChessApplication;

/** Given no arguments the application will run*/
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "cli-mode" => {
                unified_chess_gui::cli::CommandLineInterface::run();
            }
            _ => {
                println!("Unknown argument: {}", args[1]);
            }
        }
    } else {
        let mut application: ChessApplication = ChessApplication::new();

        application.run().expect("Error from iced.");
    }
}

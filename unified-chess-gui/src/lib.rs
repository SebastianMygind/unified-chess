pub mod cli;
mod fen;
mod representation;
mod svg_handling;

use iced::application;
use iced::widget::{column, container, row, svg, text};
use iced::Element;
use iced::Length::Fill;
use iced::Result;
use iced::Theme;
use iced::{self, theme};
use svg_handling::SvgPieces;
use unified_chess_shared::shared_types::{Piece, PieceType, Position, SharedState};

struct UserMove {
    start_position: Position,
    end_position: Position,
    promotion_piece: Option<PieceType>,
}

enum ColoredPieces {
    WKing,
    BKing,
    WQueen,
    BQueen,
    WRook,
    BRook,
    WBishop,
    BBishop,
    WKnight,
    BKnight,
    WPawn,
    BPawn,
}

#[derive(Default)]
pub struct ChessApplication {
    pub game_instance: Option<GameState>,
    prefered_pieces: SvgPieces,
}

impl ChessApplication {
    pub fn new() -> Self {
        Self {
            game_instance: None,
            prefered_pieces: SvgPieces::default(),
        }
    }

    //pub fn start_new_game(&mut self)
}

pub struct GameState {
    selected_square: Option<Coordinate>,
    chess_board: SharedState,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    start_position: Coordinate,
    end_position: Coordinate,
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    StartNewGame,
    QuitGame,
    ClickSquare(Coordinate),
    MakeMove(Move),
}

impl ChessApplication {
    pub fn run(&mut self) -> iced::Result {
        iced::application(Self::title, Self::update, Self::view)
            .theme(Self::theme)
            .run()
    }
    fn title(&self) -> String {
        String::from("Chess-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartNewGame => {}

            Message::QuitGame => {}

            Message::ClickSquare(coordinate) => {}

            Message::MakeMove(chess_move) => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let handle = svg::Handle::from_memory(self.prefered_pieces.white_queen);

        let svg = svg(handle).height(Fill).width(Fill);

        container(column![
            row![text("This should be left"), text("This should be right"),],
            svg
        ])
        .padding(20)
        .into()
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

const PIECES: [&str; 12] = [
    "bB.svg", "bK.svg", "bN.svg", "bP.svg", "bQ.svg", "bR.svg", "wB.svg", "wK.svg", "wN.svg",
    "wP.svg", "wQ.svg", "wR.svg",
];

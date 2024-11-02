use crate::ColoredPieces;

type Board = [[Option<ColoredPieces>; 8]; 8];

enum Perspective {
    WhitePOV,
    BlackPOV,
}

struct ChessBoard {
    board: Board,
    view_point: Perspective,
    clickable: bool,
}

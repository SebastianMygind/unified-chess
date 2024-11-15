pub mod fen_errors;

#[derive(Debug, PartialEq)]
pub enum FenArguments {
    Position,
    SideToMove,
    CastlingAbility,
    EnPassantTargetSquare,
    HalfMoveClock,
    FullMoveCounter,
}

#[derive(Debug, PartialEq)]
pub enum FenErrorKind {
    InvalidArgument(FenArguments),
    MissingArgument(FenArguments),
    TooManyArguments,
}

#[derive(Debug)]
pub struct MoveError {}

#[derive(Debug, PartialEq)]
pub struct FenError {
    pub kind: FenErrorKind,
    pub given_string: String,
}

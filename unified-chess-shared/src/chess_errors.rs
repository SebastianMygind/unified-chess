pub mod fen_errors;

pub enum FenArguments {
    Position,
    SideToMove,
    CastlingAbility,
    EnPassantTargetSquare,
    HalfMoveClock,
    FullMoveCounter,
}
pub enum FenErrorKind {
    InvalidArgument(FenArguments),
    MissingArgument(FenArguments),
    TooManyArguments,
}
pub struct MoveError {}

pub struct FenError {
    pub kind: FenErrorKind,
    pub given_string: String,
}

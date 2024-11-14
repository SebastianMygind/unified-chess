use crate::chess_errors::{FenArguments, FenError, FenErrorKind};

impl FenError {
    pub fn new(kind: FenErrorKind, given_string: &str) -> Self {
        Self {
            kind,
            given_string: given_string.to_string(),
        }
    }
    pub fn missing_argument(argument: FenArguments, fen_string: &str) -> Self {
        Self::new(FenErrorKind::MissingArgument(argument), fen_string)
    }
}

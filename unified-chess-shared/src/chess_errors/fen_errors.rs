use crate::chess_errors::{FenArguments, FenError, FenErrorKind};
use std::fmt::Formatter;

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

    pub fn parser_error(arguments: FenArguments, given_string: &str) -> Self {
        Self::new(FenErrorKind::ParserError(arguments), given_string)
    }
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

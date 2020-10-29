use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    ParseFailed,
    NoFile,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            ParseFailed | NoFile => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Error::*;

        match self {
            ParseFailed => write!(f, "parse failed"),
            NoFile => write!(f, "no input file"),
        }
    }
}

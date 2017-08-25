use std::result;
use std::fmt;
use std::error;
use std::str::Utf8Error;

/// Error enumerates the list of possible errors in this app.
#[derive(Debug, PartialEq)]
pub enum Error {
    RomajiParse,
    Utf8Error(Utf8Error),
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RomajiParse => write!(f, "failed to parse romaji"),
            Error::Utf8Error(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RomajiParse => "romaji parse error",
            Error::Utf8Error(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::RomajiParse => None,
            Error::Utf8Error(ref e) => Some(e),
        }
    }
}

/// Result is a convenient type alias that fixes the type of the error to
/// the `Error` type defined in this crate.
pub type Result<T> = result::Result<T, Error>;

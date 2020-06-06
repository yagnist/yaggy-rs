
use std::io;
use std::fmt;
use std::result::Result as StdResult;

use log;

pub(crate) type Result<T> = StdResult<T, YaggyError>;

#[derive(Debug)]
pub(crate) enum YaggyError {
    Io(io::Error),
    SetLoggerError(log::SetLoggerError),
    UnknownBasedir,
}

impl fmt::Display for YaggyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        match *self {
            Self::Io(ref e) => write!(f, "IO Error: {}", e),
            Self::SetLoggerError(ref e) => write!(f, "Logging Init Error: {}", e),
            Self::UnknownBasedir => write!(f, "Unable to get scenario base directory"),
        }
    }
}

impl From<io::Error> for YaggyError {
    fn from(e: io::Error) -> Self {
        YaggyError::Io(e)
    }
}

impl From<fern::InitError> for YaggyError {
    fn from(e: fern::InitError) -> Self {
        match e {
            fern::InitError::Io(x) => YaggyError::Io(x),
            fern::InitError::SetLoggerError(x) => YaggyError::SetLoggerError(x),
        }
    }
}

impl From<log::SetLoggerError> for YaggyError {
    fn from(e: log::SetLoggerError) -> Self {
        YaggyError::SetLoggerError(e)
    }
}

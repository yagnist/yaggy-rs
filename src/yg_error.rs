mod io_error;

use std::error;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

use crate::YgScenarioError;

pub(crate) use io_error::{YgIoError, YgIoResult};

pub(crate) type YgResult<T> = StdResult<T, YgError>;

#[derive(Debug)]
pub(crate) enum YgError {
    Io(YgIoError),
    Logging(fern::InitError),
    Scenario(YgScenarioError),
}

impl error::Error for YgError {}

impl fmt::Display for YgError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, msg) = match &self {
            YgError::Io(x) => ("I/O error".to_string(), x.to_string()),
            YgError::Scenario(err) => {
                ("Scenario error".to_string(), err.to_string())
            }
            YgError::Logging(err) => {
                ("Logging error".to_string(), err.to_string())
            }
        };

        write!(fmt, "{}: {}", prefix, msg)
    }
}

impl From<YgScenarioError> for YgError {
    fn from(e: YgScenarioError) -> YgError {
        YgError::Scenario(e)
    }
}

impl From<YgIoError> for YgError {
    fn from(e: YgIoError) -> YgError {
        YgError::Io(e)
    }
}

impl From<io::Error> for YgError {
    fn from(err: io::Error) -> YgError {
        YgError::Io(YgIoError::GenericError(err))
    }
}

impl From<fern::InitError> for YgError {
    fn from(err: fern::InitError) -> YgError {
        YgError::Logging(err)
    }
}

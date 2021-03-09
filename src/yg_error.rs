mod io_error;
mod scenario_error;

use std::error;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

pub(crate) use io_error::YgIoError;
pub(crate) use scenario_error::YgScenarioError;

pub(crate) type YgResult<T> = StdResult<T, YgError>;
pub(crate) type YgIoResult<T> = StdResult<T, YgIoError>;
pub(crate) type YgScenarioResult<T> = StdResult<T, YgScenarioError>;

#[derive(Debug)]
pub(crate) enum YgError {
    Io(YgIoError),
    Logging(fern::InitError),
    // (filename, err)
    Scenario(String, YgScenarioError),
}

impl error::Error for YgError {}

impl fmt::Display for YgError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, msg) = match &self {
            YgError::Io(x) => ("I/O error".to_string(), x.to_string()),
            YgError::Scenario(filename, err) => (
                format!("Scenario at \"{}\" error", filename),
                err.to_string(),
            ),
            YgError::Logging(err) => {
                ("Logging error".to_string(), err.to_string())
            }
        };

        write!(fmt, "{}: {}", prefix, msg)
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

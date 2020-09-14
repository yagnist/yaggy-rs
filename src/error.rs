
use std::io;
use std::fmt;
use std::result::Result as StdResult;
use std::rc::Rc;

use log;

pub(crate) type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Canonicalization {
        path: Rc<String>,
        source: io::Error,
    },
    Basedir {
        path: Rc<String>,
    },
    ScenarioFilename {
        path: Rc<String>,
    },
    ScenarioOpen {
        path: Rc<String>,
        source: io::Error,
    },
    ScenarioRead {
        path: Rc<String>,
        source: io::Error,
    },
    Logdir {
        path: String,
        source: io::Error,
    },
    Runtimedir {
        path: String,
        source: io::Error,
    },
    NotWritable {
        kind: String,
        path: String,
        source: io::Error,
    },
    Io {
        source: io::Error,
    },
    LoggingInit {
        source: log::SetLoggerError,
    },
    Syntax {
        path: Rc<String>,
        line: u32,
        message: String,
    },
    Command {
        path: Rc<String>,
        line: u32,
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (error_type, msg) = match self {
            Self::Canonicalization { path, source } => {
                ("Canonicalization", format!("Unable to get canonical path for \"{}\": {}", path, source))
            },
            Self::Basedir { path } => {
                ("Path", format!("Unable to get base directory for \"{}\"", path))
            },
            Self::ScenarioFilename { path } => {
                ("Scenario filename", format!("Invalid UTF-8 in \"{}\"", path))
            },
            Self::ScenarioOpen { path, source } => {
                ("Scenario open", format!("Error opening scenario at \"{}\": {}", path, source))
            },
            Self::ScenarioRead { path, source } => {
                ("Scenario read", format!("Error reading scenario from \"{}\": {}", path, source))
            },
            Self::Logdir { path, source } => {
                ("Logdir", format!("Unable to create logdir at \"{}\": {}", path, source))
            },
            Self::Runtimedir { path, source } => {
                ("Runtimedir", format!("Unable to create runtimedir at \"{}\": {}", path, source))
            },
            Self::NotWritable { kind, path, source } => {
                ("NotWritable", format!("{} path \"{}\" is not writable: {}", kind, path, source))
            },
            Self::Io { source } => {
                ("I/O", source.to_string())
            },
            Self::LoggingInit { source } => {
                ("Logging init", source.to_string())
            },
            Self::Syntax { path, line, message } => {
                ("Syntax", format!("{}, file: \"{}\", line: {}", message, path, line))
            },
            Self::Command { path, line, message } => {
                ("Command", format!("{}, file: \"{}\", line: {}", message, path, line))
            },
        };

        write!(f, "{} error: {}", error_type, msg)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io { source: e }
    }
}

impl From<fern::InitError> for Error {
    fn from(e: fern::InitError) -> Self {
        match e {
            fern::InitError::Io(x) => Error::Io { source: x },
            fern::InitError::SetLoggerError(x) => Error::LoggingInit { source: x },
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(e: log::SetLoggerError) -> Self {
        Error::LoggingInit { source: e }
    }
}

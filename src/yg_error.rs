use std::error;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

pub(crate) type YgResult<T> = StdResult<T, YgError>;

pub(crate) struct YgError {
    repr: Repr,
}

enum Repr {
    Scenario {
        kind: ScenarioKind,
        filename: String,
        line_num: u32,
        message: String,
        source: Option<io::Error>,
    },
    Io {
        message: String,
        // message: &'static str,
        source: Option<io::Error>,
        // source: Box<dyn error::Error + Send + Sync>,
    },
    Logging {
        source: Box<dyn error::Error + Send + Sync>,
    },
}

enum ScenarioKind {
    Syntax,
    Exec,
}

impl ScenarioKind {
    fn as_str(&self) -> &str {
        match *self {
            ScenarioKind::Syntax => "Scenario syntax error",
            ScenarioKind::Exec => "Scenario command execution error",
        }
    }
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl YgError {
    pub(crate) fn scenario_syntax_error(
        filename: String,
        line_num: u32,
        message: String,
        source: Option<io::Error>,
    ) -> YgError {
        YgError {
            repr: Repr::Scenario {
                kind: ScenarioKind::Syntax,
                filename,
                line_num,
                message,
                source,
            },
        }
    }
    pub(crate) fn scenario_command_error(
        filename: String,
        line_num: u32,
        message: String,
        source: Option<io::Error>,
    ) -> YgError {
        YgError {
            repr: Repr::Scenario {
                kind: ScenarioKind::Exec,
                filename,
                line_num,
                message,
                source,
            },
        }
    }
    pub(crate) fn io_error(message: String) -> YgError {
        YgError { repr: Repr::Io { message: message, source: None } }
    }
    pub(crate) fn io_error_with_source(
        message: String,
        source: io::Error,
    ) -> YgError {
        YgError { repr: Repr::Io { message, source: Some(source) } }
    }
    fn from_fern(source: fern::InitError) -> YgError {
        YgError { repr: Repr::Logging { source: Box::new(source) } }
    }
    fn from_log(source: log::SetLoggerError) -> YgError {
        YgError { repr: Repr::Logging { source: Box::new(source) } }
    }
}

impl fmt::Display for YgError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, msg) = match &self.repr {
            Repr::Scenario {
                ref kind,
                filename,
                line_num,
                message,
                source: ref _source,
            } => (
                kind.to_string(),
                format!(
                    "{}, file: \"{}\", line: {}",
                    message, filename, line_num
                ),
            ),
            Repr::Io { message, ref source } => match source {
                Some(err) => (message.clone(), err.to_string()),
                None => ("I/O error".to_string(), message.to_string()),
            },
            Repr::Logging { ref source } => {
                ("Logging init error".to_string(), source.to_string())
            }
        };

        write!(fmt, "{}: {}", prefix, msg)
    }
}

// impl error::Error for YgError {}

impl From<io::Error> for YgError {
    fn from(e: io::Error) -> YgError {
        YgError::io_error_with_source("I/O error".to_string(), e)
    }
}

impl From<fern::InitError> for YgError {
    fn from(e: fern::InitError) -> YgError {
        YgError::from_fern(e)
    }
}

impl From<log::SetLoggerError> for YgError {
    fn from(e: log::SetLoggerError) -> YgError {
        YgError::from_log(e)
    }
}

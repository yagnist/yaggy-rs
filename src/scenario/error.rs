use std::fmt;
use std::rc::Rc;
use std::result::Result;

pub(crate) type YgScenarioResult<T> = Result<T, YgScenarioError>;

#[derive(Debug, Clone)]
pub(crate) enum YgScenarioError {
    OpenError(Rc<String>, String),
    ReadError(Rc<String>, String),
    // (filename, line_num, message)
    IoError(Rc<String>, u32, String),
    // (filename, line_num, message)
    SyntaxError(Rc<String>, u32, String),
    // (filename, line_num, message)
    IncludeError(Rc<String>, u32, String),
}

impl fmt::Display for YgScenarioError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use YgScenarioError::*;
        match self {
            OpenError(filename, message) => {
                write!(
                    fmt,
                    "unable to open scenario {}\n{}",
                    filename, message
                )
            }
            ReadError(filename, message) => {
                write!(
                    fmt,
                    "unable to read scenario {}\n{}",
                    filename, message
                )
            }
            IoError(filename, line_num, message) => {
                write!(fmt, "file:{} line:{}\n{}", filename, line_num, message)
            }
            SyntaxError(filename, line_num, message) => {
                write!(
                    fmt,
                    "file:{} line:{} message:{}",
                    filename, line_num, message
                )
            }
            IncludeError(filename, line_num, message) => {
                write!(
                    fmt,
                    "file:{} line:{} INCLUDE error\n{}",
                    filename, line_num, message
                )
            }
        }
    }
}

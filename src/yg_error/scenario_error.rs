use std::fmt;
use std::io;

#[derive(Debug)]
pub(crate) enum YgScenarioError {
    OpenError(io::Error),
    ReadError(io::Error),
    // (line_num, message)
    SyntaxError(u32, String),
    // (line_num)
    IncludeError(u32),
    // (line_num, smth)
    UnicodeError(u32, String),
}

impl fmt::Display for YgScenarioError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use YgScenarioError::*;
        match self {
            OpenError(err) => write!(fmt, "unable to open scenario\n{}", err),
            ReadError(err) => write!(fmt, "unable to read scenario\n{}", err),
            SyntaxError(line_num, message) => {
                write!(fmt, "line:{} message:{}", line_num, message)
            }
            IncludeError(line_num) => {
                write!(fmt, "line:{} INCLUDE error", line_num)
            }
            UnicodeError(line_num, smth) => {
                write!(fmt, "line:{} Invalid UTF-8 in \"{}\"", line_num, smth)
            }
        }
    }
}

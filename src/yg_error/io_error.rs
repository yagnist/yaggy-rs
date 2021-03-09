use std::fmt;
use std::io;

#[derive(Debug)]
pub(crate) enum YgIoError {
    // (err)
    GenericError(io::Error),
    // (path)
    BaseDirError(String),
    // (path, err)
    CanonicalPathError(String, io::Error),
    // (context, path, err)
    CreateDirError(String, String, io::Error),
    // (context, cmd, err)
    ExecError(String, String, io::Error),
    // (context, path, err)
    NotWritableError(String, String, io::Error),
    // (smth)
    UnicodeError(String),
}

impl fmt::Display for YgIoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use YgIoError::*;
        match self {
            BaseDirError(path) => {
                write!(fmt, "Unable to get base directory for \"{}\"", path)
            }
            CanonicalPathError(path, err) => write!(
                fmt,
                "Unable to get canonical path for \"{}\"\n{}",
                path, err
            ),
            CreateDirError(context, path, err) => write!(
                fmt,
                "[{}] Unable to create directory at \"{}\"\n{}",
                context, path, err
            ),
            ExecError(context, cmd, err) => write!(
                fmt,
                "[{}] Failed to execute \"{}\"\n{}",
                context, cmd, err
            ),
            GenericError(err) => write!(fmt, "{}", err.to_string()),
            NotWritableError(context, path, err) => write!(
                fmt,
                "[{}] Path \"{}\" is not writable\n{}",
                context, path, err
            ),
            UnicodeError(smth) => {
                write!(fmt, "Invalid UTF-8 in \"{}\"", smth)
            }
        }
    }
}

impl From<io::Error> for YgIoError {
    fn from(err: io::Error) -> YgIoError {
        YgIoError::GenericError(err)
    }
}

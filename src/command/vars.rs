
use std::path::Path;
use std::rc::Rc;

use crate::{Error, Result, YgPath};
use super::{Command, validators};


fn validate_args(command: &Command) -> Result<()> {
    let args = &command.parsed.args;
    let cmd_filename = Path::new(command.filename.as_str());
    let basedir = cmd_filename.yg_basedir()?;
    let filename = basedir.join(args).yg_canonicalize();

    match filename {
        Ok(_) => Ok(()),
        Err(_) => {
            let words = shell_words::split(args)
                .map_err(|_| Error::Syntax {
                    path: Rc::clone(&command.filename),
                    line: command.line_num,
                    message: format!("Invalid syntax in shell command: \"{}\"", args)
                })?;
            let _cmd = which::which(&words[0])
                .map_err(|e| Error::Syntax {
                    path: Rc::clone(&command.filename),
                    line: command.line_num,
                    message: format!("{} for shell command: \"{}\"", e.to_string(), args)
                })?;
            Ok(())
        }
    }
}

pub(crate) fn validate(command: &Command) -> Result<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    validate_args(&command)?;

    Ok(())
}

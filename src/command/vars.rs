use std::path::Path;

use super::{validators, Command};
use crate::{YgError, YgPath, YgResult};

fn validate_args(command: &Command) -> YgResult<()> {
    let args = &command.parsed.args;
    let cmd_filename = Path::new(&command.filename);
    let basedir = cmd_filename.yg_basedir()?;
    let filename = basedir.join(args).yg_canonicalize();

    match filename {
        Ok(_) => Ok(()),
        Err(_) => {
            let words = shell_words::split(args).map_err(|_| {
                YgError::scenario_syntax_error(
                    command.filename.clone(),
                    command.line_num,
                    format!("Invalid syntax in shell command: \"{}\"", args),
                    None, // FIXME
                )
            })?;
            let _cmd = which::which(&words[0]).map_err(|e| {
                YgError::scenario_syntax_error(
                    command.filename.clone(),
                    command.line_num,
                    format!(
                        "{} for shell command: \"{}\"",
                        e.to_string(),
                        args
                    ),
                    None, // FIXME
                )
            })?;
            Ok(())
        }
    }
}

pub(crate) fn validate(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    validate_args(&command)?;

    Ok(())
}

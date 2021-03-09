use std::path::Path;

use super::{validators, Command};
use crate::{YgPath, YgScenarioError, YgScenarioResult};

fn validate_args(basedir: &Path, command: &Command) -> YgScenarioResult<()> {
    let args = &command.parsed.args;
    let filename = basedir.join(args).yg_canonicalize();

    match filename {
        Ok(_) => Ok(()),
        Err(_) => {
            let words = shell_words::split(args).map_err(|_| {
                YgScenarioError::SyntaxError(
                    command.line_num,
                    format!("Invalid syntax in shell command: \"{}\"", args),
                )
            })?;
            let _cmd = which::which(&words[0]).map_err(|err| {
                YgScenarioError::SyntaxError(
                    command.line_num,
                    format!(
                        "{} for shell command: \"{}\"",
                        err.to_string(),
                        args
                    ),
                )
            })?;
            Ok(())
        }
    }
}

pub(crate) fn validate(
    basedir: &Path,
    command: &Command,
) -> YgScenarioResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    validate_args(&basedir, &command)?;

    Ok(())
}

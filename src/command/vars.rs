use std::path::Path;

use super::{Command, Validators};
use crate::{YgPath, YgScenarioError, YgScenarioResult};

pub(crate) fn validate(
    command: &Command,
    basedir: &Path,
) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?.has_args()?;

    command.validate_args(&basedir)?;

    Ok(())
}

impl Command {
    fn validate_args(&self, basedir: &Path) -> YgScenarioResult<()> {
        let args = &self.parsed.args;
        let filename = basedir.join(args).yg_canonicalize();

        match filename {
            Ok(_) => Ok(()),
            Err(_) => {
                let words = shell_words::split(args).map_err(|_| {
                    YgScenarioError::SyntaxError(
                        self.line_num,
                        format!(
                            "Invalid syntax in shell command: \"{}\"",
                            args
                        ),
                    )
                })?;
                let _cmd = which::which(&words[0]).map_err(|err| {
                    YgScenarioError::SyntaxError(
                        self.line_num,
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
}

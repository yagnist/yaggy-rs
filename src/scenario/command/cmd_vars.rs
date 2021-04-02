use std::any::Any;
use std::fmt;
use std::path::Path;
use std::rc::Rc;

use super::{Cmd, ParsedLine};
use crate::{YgPath, YgScenarioError, YgScenarioResult};

#[derive(Debug)]
enum Mode {
    Vars,
    Secrets,
}

#[derive(Debug)]
pub struct CmdVars {
    filename: Rc<String>,
    line_num: u32,
    mode: Mode,
    args: String,
}

impl fmt::Display for CmdVars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cmd = match self.mode {
            Mode::Vars => "VARS",
            Mode::Secrets => "SECRETS",
        };
        write!(f, "{} {}", cmd, self.args)
    }
}

impl Cmd for CmdVars {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self) -> YgScenarioResult<()> {
        self.validate_args()
    }
}

impl CmdVars {
    pub fn new_vars(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdVars {
            filename: Rc::clone(filename),
            line_num: line_num,
            mode: Mode::Vars,
            args: parsed.args.to_string(),
        }
    }
    pub fn new_secrets(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdVars {
            filename: Rc::clone(filename),
            line_num: line_num,
            mode: Mode::Secrets,
            args: parsed.args.to_string(),
        }
    }

    fn validate_args(&self) -> YgScenarioResult<()> {
        let args = &self.args;
        let basedir =
            Path::new(self.filename.as_str()).yg_basedir().map_err(|err| {
                YgScenarioError::IoError(
                    Rc::clone(&self.filename),
                    self.line_num,
                    err.to_string(),
                )
            })?;
        let filename = basedir.join(args).yg_canonicalize();

        match filename {
            Ok(_) => Ok(()),
            Err(_) => {
                let words = shell_words::split(args).map_err(|_| {
                    YgScenarioError::SyntaxError(
                        Rc::clone(&self.filename),
                        self.line_num,
                        format!(
                            "Invalid syntax in shell command: \"{}\"",
                            args
                        ),
                    )
                })?;
                let _cmd = which::which(&words[0]).map_err(|err| {
                    YgScenarioError::SyntaxError(
                        Rc::clone(&self.filename),
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

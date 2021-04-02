mod cmd_connect;
mod cmd_echo;
mod cmd_fetch;
mod cmd_include;
mod cmd_run;
mod cmd_sync;
mod cmd_tag;
mod cmd_vars;
mod parser;

use std::any::Any;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;

use crate::{YgScenarioError, YgScenarioResult};
use cmd_connect::{CmdConnect, CmdDisconnect, CmdReconnect, CmdReconnectIf};
use cmd_echo::CmdEcho;
use cmd_fetch::CmdFetch;
pub use cmd_include::CmdInclude;
use cmd_run::{CmdRun, CmdRunIf};
use cmd_sync::CmdSync;
use cmd_tag::{CmdTag, CmdUntag};
use cmd_vars::CmdVars;
pub use parser::{ParseError, ParsedLine};

pub(crate) trait Cmd
where
    Self: Any,
    Self: fmt::Display,
{
    fn as_any(&self) -> &dyn Any;
    fn validate(&self) -> YgScenarioResult<()> {
        Ok(())
    }
    fn run(&self) -> YgScenarioResult<()> {
        Ok(())
    }
}

pub(crate) fn from_line(
    filename: &Rc<String>,
    line_num: u32,
    line: &str,
) -> YgScenarioResult<Box<dyn Cmd>> {
    let syntax_err = |err: String| {
        YgScenarioError::SyntaxError(Rc::clone(filename), line_num, err)
    };

    let parsed: ParsedLine = line.parse().map_err(|err: ParseError| {
        YgScenarioError::SyntaxError(
            Rc::clone(filename),
            line_num,
            err.to_string(),
        )
    })?;

    let command = parsed.command.as_str();
    match command {
        "CONNECT" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            // TODO parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdConnect::new(filename, line_num, &parsed)))
        }
        "DISCONNECT" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_no_args().map_err(syntax_err)?;
            Ok(Box::new(CmdDisconnect::new(filename, line_num, &parsed)))
        }
        "RECONNECT" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            Ok(Box::new(CmdReconnect::new(filename, line_num, &parsed)))
        }
        "RECONNECT?" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            Ok(Box::new(CmdReconnectIf::new(filename, line_num, &parsed)))
        }
        "ECHO" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdEcho::new(filename, line_num, &parsed)))
        }
        "FETCH" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdFetch::new(filename, line_num, &parsed)))
        }
        "INCLUDE" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdInclude::new(filename, line_num, &parsed)))
        }
        "RUN" => {
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRun::new_remote(filename, line_num, &parsed)))
        }
        "RUN!" => {
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRun::new_remote_exclamation(
                filename, line_num, &parsed,
            )))
        }
        "LRUN" => {
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRun::new_local(filename, line_num, &parsed)))
        }
        "LRUN!" => {
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRun::new_local_exclamation(
                filename, line_num, &parsed,
            )))
        }
        "FAILED?" => {
            parsed.has_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRunIf::new_remote_if_failed(
                filename, line_num, &parsed,
            )))
        }
        "SUCCEED?" => {
            parsed.has_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRunIf::new_remote_if_succeed(
                filename, line_num, &parsed,
            )))
        }
        "LFAILED?" => {
            parsed.has_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRunIf::new_local_if_failed(
                filename, line_num, &parsed,
            )))
        }
        "LSUCCEED?" => {
            parsed.has_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdRunIf::new_local_if_succeed(
                filename, line_num, &parsed,
            )))
        }
        "SYNC" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_no_args().map_err(syntax_err)?;
            Ok(Box::new(CmdSync::new(filename, line_num, &parsed)))
        }
        "TAG" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdTag::new(filename, line_num, &parsed)))
        }
        "UNTAG" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdUntag::new(filename, line_num, &parsed)))
        }
        "VARS" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdVars::new_vars(filename, line_num, &parsed)))
        }
        "SECRETS" => {
            parsed.has_no_reference().map_err(syntax_err)?;
            parsed.has_no_back_reference().map_err(syntax_err)?;
            parsed.has_args().map_err(syntax_err)?;
            Ok(Box::new(CmdVars::new_secrets(filename, line_num, &parsed)))
        }
        _ => {
            let message =
                format!("Unknown scenario command: \"{}\"", line.clone());
            Err(YgScenarioError::SyntaxError(
                Rc::clone(filename),
                line_num,
                message,
            ))
        }
    }
}

pub(crate) fn load_main(
    filename: &str,
) -> Vec<YgScenarioResult<Box<dyn Cmd>>> {
    let parsed = ParsedLine::new("", "", "", filename);
    let to_include =
        CmdInclude::new(&Rc::new(filename.to_string()), 0, &parsed);
    load_included(&to_include)
}

fn load_included(
    to_include: &CmdInclude,
) -> Vec<YgScenarioResult<Box<dyn Cmd>>> {
    let mut commands: Vec<YgScenarioResult<Box<dyn Cmd>>> = Vec::new();
    for cmd in to_include.load() {
        match cmd {
            Ok(command) => {
                if let Some(cmd) =
                    command.as_any().downcast_ref::<CmdInclude>()
                {
                    let mut included = load_included(&cmd);
                    commands.push(Ok(command));
                    commands.append(&mut included);
                } else {
                    commands.push(Ok(command));
                }
            }
            Err(command) => commands.push(Err(command)),
        }
    }
    commands
}

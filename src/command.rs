mod connect;
mod echo;
mod fetch;
mod include;
mod run;
mod sync;
mod tag;
mod vars;

mod validators;

use std::fmt;

use crate::{ParsedLine, YgError, YgResult};

#[derive(Debug,PartialEq)]
pub enum Cmd {
    Connect,
    Reconnect,
    ReconnectIf,
    Disconnect,
    Echo,
    Fetch,
    Include,
    Sync,
    Tag,
    Untag,
    Run,
    RunExclamation,
    LRun,
    LRunExclamation,
    Succeed,
    LSucceed,
    Failed,
    LFailed,
    Vars,
    Secrets,
}

#[derive(Debug)]
pub(crate) struct Command {
    // filename: Rc<String>,
    pub filename: String,
    pub line_num: u32,
    pub parsed: ParsedLine,
    cmd: Cmd,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.display().as_str())
        // write!(f, "{}", self.display())
    }
}

impl Command {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
    pub(crate) fn validate(&self) -> YgResult<()> {
        use Cmd::*;

        match self.cmd {
            Connect | Disconnect => connect::validate(&self),
            Reconnect => connect::validate_reconnect(&self),
            ReconnectIf => connect::validate_reconnect_if(&self),
            Vars | Secrets => vars::validate(&self),
            Sync => sync::validate(&self),
            Echo => echo::validate(&self),
            Fetch => fetch::validate(&self),
            Include => include::validate(&self),
            Tag => tag::validate_tag(&self),
            Untag => tag::validate_untag(&self),
            Run | RunExclamation | LRun | LRunExclamation => {
                run::validate_run(&self)
            }
            Succeed | LSucceed | Failed | LFailed => {
                run::validate_conditional(&self)
            }
        }
    }
    pub(crate) fn is_include(&self) -> bool {
        self.cmd == Cmd::Include
    }
}

pub(crate) struct CommandBuilder;

impl CommandBuilder {
    pub fn from_parsed_line(
        parsed: ParsedLine,
        filename: &str,
        line_num: u32,
    ) -> YgResult<Command> {
        use Cmd::*;

        let cmd = match parsed.command.as_str() {
            "CONNECT" => Connect,
            "RECONNECT" => Reconnect,
            "RECONNECT?" => ReconnectIf,
            "DISCONNECT" => Disconnect,
            "ECHO" => Echo,
            "FETCH" => Fetch,
            "INCLUDE" => Include,
            "RUN" => Run,
            "RUN!" => RunExclamation,
            "LRUN" => LRun,
            "LRUN!" => LRunExclamation,
            "SUCCEED?" => Succeed,
            "FAILED?" => Failed,
            "LSUCCEED?" => LSucceed,
            "LFAILED?" => LFailed,
            "SYNC" => Sync,
            "TAG" => Tag,
            "UNTAG" => Untag,
            "VARS" => Vars,
            "SECRETS" => Secrets,
            x => {
                let message =
                    format!("Unknown scenario command: \"{}\"", x.clone());
                return Err(YgError::scenario_syntax_error(
                    filename.to_string(),
                    line_num,
                    message,
                    None, // FIXME
                ));
            }
        };

        Ok(Command {
            filename: filename.to_string(),
            line_num: line_num,
            parsed: parsed,
            cmd: cmd,
        })
    }
}

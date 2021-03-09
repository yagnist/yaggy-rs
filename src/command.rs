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
use std::path::Path;

use crate::{ParsedLine, YgScenarioError, YgScenarioResult};

#[derive(Debug, PartialEq)]
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
    ) -> YgScenarioResult<Command> {
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
                return Err(YgScenarioError::SyntaxError(line_num, message));
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

pub(crate) fn validate_command(
    basedir: &Path,
    command: &Command,
) -> YgScenarioResult<()> {
    use Cmd::*;

    match command.cmd {
        Connect | Disconnect => connect::validate(&command),
        Reconnect => connect::validate_reconnect(&command),
        ReconnectIf => connect::validate_reconnect_if(&command),
        Vars | Secrets => vars::validate(&basedir, &command),
        Sync => sync::validate(&command),
        Echo => echo::validate(&command),
        Fetch => fetch::validate(&command),
        Include => include::validate(&command),
        Tag => tag::validate_tag(&command),
        Untag => tag::validate_untag(&command),
        Run | RunExclamation | LRun | LRunExclamation => {
            run::validate_run(&command)
        }
        Succeed | LSucceed | Failed | LFailed => {
            run::validate_conditional(&command)
        }
    }
}

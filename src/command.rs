
mod connect;
mod echo;
mod fetch;
mod include;
mod run;
mod sync;
mod tag;
mod vars;

use std::fmt;
use std::rc::Rc;

use crate::Error;
use crate::Result;
use crate::ParsedLine;

use connect::CmdConnect;
use echo::CmdEcho;
use fetch::CmdFetch;
use include::CmdInclude;
use run::CmdRun;
use sync::CmdSync;
use tag::CmdTag;
use vars::CmdVars;



pub(crate) trait Command {
    fn run(&self) -> crate::Result<()> {
        Ok(())
    }
    fn display(&self) -> String;
}

impl fmt::Display for dyn Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.display().as_str())
        // write!(f, "{}", self.display())
    }
}


pub(crate) struct CommandBuilder;

impl CommandBuilder {

    pub fn from_parsed_line(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Result<Box<dyn Command>> {
        match parsed.command.as_str() {
            "CONNECT" | "RECONNECT" | "RECONNECT?" | "DISCONNECT" => Ok(Box::new(CmdConnect::new(parsed, filename, line_num))),
            "ECHO" => Ok(Box::new(CmdEcho::new(parsed, filename, line_num))),
            "FETCH" => Ok(Box::new(CmdFetch::new(parsed, filename, line_num))),
            "INCLUDE" => Ok(Box::new(CmdInclude::new(parsed, filename, line_num))),
            "RUN" | "RUN!" | "LRUN" | "LRUN!" | "SUCCEED?" | "FAILED?" | "LSUCCEED?" | "LFAILED?" => Ok(Box::new(CmdRun::new(parsed, filename, line_num))),
            "SYNC" => Ok(Box::new(CmdSync::new(parsed, filename, line_num))),
            "TAG" | "UNTAG" => Ok(Box::new(CmdTag::new(parsed, filename, line_num))),
            "VARS" | "SECRETS" => Ok(Box::new(CmdVars::new(parsed, filename, line_num))),
            x => Err(Error::Command{ path: Rc::clone(filename), line: line_num, message: format!("unknown command: {}", x) }),
        }
    }

}

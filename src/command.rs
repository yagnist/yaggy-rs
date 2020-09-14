
mod run;

use std::fmt;
use std::rc::Rc;

use crate::Error;
use crate::Result;
use crate::ParsedLine;

use run::CmdRun;



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
            "RUN" | "RUN!" | "LRUN" | "LRUN!" | "SUCCEED?" | "FAILED?" | "LSUCCEED?" | "LFAILED?" => Ok(Box::new(CmdRun::new(parsed, filename, line_num))),
            x => Err(Error::Command{ path: Rc::clone(filename), line: line_num, message: format!("unknown command: {}", x) }),
        }
    }

}

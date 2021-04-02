use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub struct CmdEcho {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdEcho {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECHO {}", self.args)
    }
}

impl Cmd for CmdEcho {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdEcho {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdEcho {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

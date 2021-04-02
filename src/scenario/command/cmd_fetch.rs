use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub struct CmdFetch {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdFetch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FETCH {}", self.args)
    }
}

impl Cmd for CmdFetch {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdFetch {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdFetch {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

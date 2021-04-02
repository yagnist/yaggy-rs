use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub struct CmdTag {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TAG {}", self.args)
    }
}

impl Cmd for CmdTag {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdTag {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdTag {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CmdUntag {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdUntag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UNTAG {}", self.args)
    }
}

impl Cmd for CmdUntag {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdUntag {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdUntag {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

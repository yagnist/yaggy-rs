use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub struct CmdSync {
    filename: Rc<String>,
    line_num: u32,
}

impl fmt::Display for CmdSync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SYNC")
    }
}

impl Cmd for CmdSync {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdSync {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        _parsed: &ParsedLine,
    ) -> Self {
        CmdSync { filename: Rc::clone(filename), line_num: line_num }
    }
}

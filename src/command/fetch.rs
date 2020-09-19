
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
pub struct CmdFetch {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
}

impl CmdFetch {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        CmdFetch {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
        }
    }
}

impl Command for CmdFetch {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}


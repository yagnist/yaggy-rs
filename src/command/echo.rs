
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
pub struct CmdEcho {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
}

impl CmdEcho {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        CmdEcho {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
        }
    }
}

impl Command for CmdEcho {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}


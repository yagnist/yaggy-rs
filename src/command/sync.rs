
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
pub struct CmdSync {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
}

impl CmdSync {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        CmdSync {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
        }
    }
}

impl Command for CmdSync {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}


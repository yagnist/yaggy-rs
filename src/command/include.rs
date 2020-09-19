
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
pub struct CmdInclude {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
}

impl CmdInclude {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        CmdInclude {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
        }
    }
}

impl Command for CmdInclude {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}



use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
enum Mode {
    Tag,
    Untag,
}


#[derive(Debug)]
pub struct CmdTag {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
    mode: Mode,
}

impl CmdTag {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        let mode = if parsed.command == "TAG" { Mode::Tag } else { Mode::Untag };
        CmdTag {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
            mode: mode,
        }
    }
}

impl Command for CmdTag {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}


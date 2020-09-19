
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
enum Mode {
    Vars,
    Secrets,
}


#[derive(Debug)]
pub struct CmdVars {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
    mode: Mode,
}

impl CmdVars {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        let mode = if parsed.command == "VARS" { Mode::Vars } else { Mode::Secrets };
        CmdVars {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
            mode: mode,
        }
    }
}

impl Command for CmdVars {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}

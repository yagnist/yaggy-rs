
use std::rc::Rc;

use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
enum Mode {
    Connect,
    Reconnect,
    Disconnect,
}


#[derive(Debug)]
pub struct CmdConnect {
    filename: Rc<String>,
    line_num: u32,
    parsed: ParsedLine,
    mode: Mode,
    is_conditional: bool,
}

impl CmdConnect {
    pub fn new(parsed: ParsedLine, filename: &Rc<String>, line_num: u32) -> Self {
        let mode = match parsed.command.as_str() {
            "CONNECT" => Mode::Connect,
            "RECONNECT" | "RECONNECT?"=> Mode::Reconnect,
            "DISCONNECT" => Mode::Disconnect,
            _ => unreachable!(),
        };
        let is_conditional = parsed.command.ends_with('?');

        CmdConnect {
            filename: Rc::clone(filename),
            line_num: line_num,
            parsed: parsed,
            mode: mode,
            is_conditional: is_conditional,
        }
    }
}

impl Command for CmdConnect {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}


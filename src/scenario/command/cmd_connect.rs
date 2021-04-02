use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub struct CmdConnect {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdConnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CONNECT {}", self.args)
    }
}

impl Cmd for CmdConnect {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdConnect {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdConnect {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CmdDisconnect {
    filename: Rc<String>,
    line_num: u32,
}

impl fmt::Display for CmdDisconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DISCONNECT")
    }
}

impl Cmd for CmdDisconnect {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdDisconnect {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        _parsed: &ParsedLine,
    ) -> Self {
        CmdDisconnect { filename: Rc::clone(filename), line_num: line_num }
    }
}

#[derive(Debug)]
pub struct CmdReconnect {
    filename: Rc<String>,
    line_num: u32,
    args: String,
}

impl fmt::Display for CmdReconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RECONNECT {}", self.args)
    }
}

impl Cmd for CmdReconnect {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdReconnect {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdReconnect {
            filename: Rc::clone(filename),
            line_num: line_num,
            args: parsed.args.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CmdReconnectIf {
    filename: Rc<String>,
    line_num: u32,
    back_reference: String,
    args: String,
}

impl fmt::Display for CmdReconnectIf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RECONNECT? {} {}", self.back_reference, self.args)
    }
}

impl Cmd for CmdReconnectIf {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdReconnectIf {
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdReconnectIf {
            filename: Rc::clone(filename),
            line_num: line_num,
            back_reference: parsed.back_reference.to_string(),
            args: parsed.args.to_string(),
        }
    }
}

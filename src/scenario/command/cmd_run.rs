use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::{Cmd, ParsedLine};

#[derive(Debug)]
pub enum Mode {
    Local,
    Remote,
}

#[derive(Debug)]
pub enum RunIf {
    Failed,
    Succeed,
}

#[derive(Debug)]
pub struct CmdRun {
    filename: Rc<String>,
    line_num: u32,
    reference: String,
    args: String,
    mode: Mode,
    can_fail: bool,
}

impl fmt::Display for CmdRun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cmd = match self.mode {
            Mode::Local => match self.can_fail {
                true => "LRUN!",
                false => "LRUN",
            },
            Mode::Remote => match self.can_fail {
                true => "RUN!",
                false => "RUN",
            },
        };
        let reference = match self.reference.is_empty() {
            true => "".to_string(),
            false => format!("{} ", self.reference),
        };
        write!(f, "{}{} {}", reference, cmd, self.args)
    }
}

impl Cmd for CmdRun {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdRun {
    pub fn new_local(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRun {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Local,
            can_fail: false,
        }
    }
    pub fn new_local_exclamation(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRun {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Local,
            can_fail: true,
        }
    }
    pub fn new_remote(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRun {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Remote,
            can_fail: false,
        }
    }
    pub fn new_remote_exclamation(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRun {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Remote,
            can_fail: true,
        }
    }
}

#[derive(Debug)]
pub struct CmdRunIf {
    filename: Rc<String>,
    line_num: u32,
    reference: String,
    back_reference: String,
    args: String,
    mode: Mode,
    run_if: RunIf,
}

impl fmt::Display for CmdRunIf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cmd = match self.mode {
            Mode::Local => match self.run_if {
                RunIf::Failed => "LFAILED?",
                RunIf::Succeed => "LSUCCEED?",
            },
            Mode::Remote => match self.run_if {
                RunIf::Failed => "FAILED?",
                RunIf::Succeed => "SUCCEED?",
            },
        };
        let reference = match self.reference.is_empty() {
            true => "".to_string(),
            false => format!("{} ", self.reference),
        };
        let back_reference = match self.back_reference.is_empty() {
            true => "".to_string(),
            false => format!(" {}", self.back_reference),
        };
        write!(f, "{}{}{} {}", reference, cmd, back_reference, self.args)
    }
}

impl Cmd for CmdRunIf {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CmdRunIf {
    pub fn new_local_if_failed(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRunIf {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            back_reference: parsed.back_reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Local,
            run_if: RunIf::Failed,
        }
    }
    pub fn new_local_if_succeed(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRunIf {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            back_reference: parsed.back_reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Local,
            run_if: RunIf::Succeed,
        }
    }
    pub fn new_remote_if_failed(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRunIf {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            back_reference: parsed.back_reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Remote,
            run_if: RunIf::Failed,
        }
    }
    pub fn new_remote_if_succeed(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        CmdRunIf {
            filename: Rc::clone(filename),
            line_num: line_num,
            reference: parsed.reference.to_string(),
            back_reference: parsed.back_reference.to_string(),
            args: parsed.args.to_string(),
            mode: Mode::Remote,
            run_if: RunIf::Succeed,
        }
    }
}

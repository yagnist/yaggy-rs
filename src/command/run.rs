
use crate::ParsedLine;
use super::Command;


#[derive(Debug)]
enum Condition {
    OnFailure,
    OnSuccess,
}

#[derive(Debug)]
pub struct CmdRun {
    parsed: ParsedLine,
    is_local: bool,
    can_fail: bool,
    is_conditional: bool,
    condition: Option<Condition>,
}

impl CmdRun {
    pub fn new(parsed: ParsedLine) -> Self {
        let is_local = parsed.command.starts_with("L");
        let can_fail = parsed.command.ends_with('!');
        let is_conditional = parsed.command.ends_with('?');
        let condition = match parsed.command.as_str() {
            "SUCCEED?" | "LSUCCEED?" => Some(Condition::OnSuccess),
            "FAILED?" | "LFAILED?" => Some(Condition::OnFailure),
            _ => None,
        };

        CmdRun {
            parsed: parsed,
            is_local: is_local,
            can_fail: can_fail,
            is_conditional: is_conditional,
            condition: condition,
        }
    }
}

impl Command for CmdRun {
    fn display(&self) -> String {
        format!("{}", self.parsed)
    }
}

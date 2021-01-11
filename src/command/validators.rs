
use crate::{YgResult, YgError};
use super::Command;


pub(crate) fn has_no_reference(command: &Command) -> YgResult<()> {
    if command.parsed.has_reference() {
        return Err(YgError::scenario_syntax_error(
                command.filename.clone(),
                command.line_num,
                "Reference is not allowed for this command".to_string(),
                None,  // FIXME
        ));
    }
    Ok(())
}

pub(crate) fn has_no_back_reference(command: &Command) -> YgResult<()> {
    if command.parsed.has_back_reference() {
        return Err(YgError::scenario_syntax_error(
                command.filename.clone(),
                command.line_num,
                "Back reference is not allowed for this command".to_string(),
                None,  // FIXME
        ));
    }
    Ok(())
}

pub(crate) fn has_args(command: &Command) -> YgResult<()> {
    if !command.parsed.has_args() {
        return Err(YgError::scenario_syntax_error(
                command.filename.clone(),
                command.line_num,
                "This command requires some arguments".to_string(),
                None,  // FIXME
        ));
    }
    Ok(())
}

pub(crate) fn has_no_args(command: &Command) -> YgResult<()> {
    if command.parsed.has_args() {
        return Err(YgError::scenario_syntax_error(
                command.filename.clone(),
                command.line_num,
                "This command does not expect any arguments".to_string(),
                None,  // FIXME
        ));
    }
    Ok(())
}

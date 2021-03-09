use super::Command;
use crate::{YgScenarioError, YgScenarioResult};

pub(crate) fn has_no_reference(command: &Command) -> YgScenarioResult<()> {
    if command.parsed.has_reference() {
        return Err(YgScenarioError::SyntaxError(
            command.line_num,
            "Reference is not allowed for this command".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn has_no_back_reference(
    command: &Command,
) -> YgScenarioResult<()> {
    if command.parsed.has_back_reference() {
        return Err(YgScenarioError::SyntaxError(
            command.line_num,
            "Back reference is not allowed for this command".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn has_args(command: &Command) -> YgScenarioResult<()> {
    if !command.parsed.has_args() {
        return Err(YgScenarioError::SyntaxError(
            command.line_num,
            "This command requires some arguments".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn has_no_args(command: &Command) -> YgScenarioResult<()> {
    if command.parsed.has_args() {
        return Err(YgScenarioError::SyntaxError(
            command.line_num,
            "This command does not expect any arguments".to_string(),
        ));
    }
    Ok(())
}

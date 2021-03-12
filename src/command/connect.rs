use super::{Command, Validators};
use crate::YgScenarioResult;

pub(crate) fn validate(command: &Command) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?.has_no_args()?;

    Ok(())
}

pub(crate) fn validate_reconnect(command: &Command) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?;

    // TODO
    // validate reconnect

    Ok(())
}

pub(crate) fn validate_reconnect_if(
    command: &Command,
) -> YgScenarioResult<()> {
    command.has_no_reference()?;

    // TODO
    // validate reconnect_if

    Ok(())
}

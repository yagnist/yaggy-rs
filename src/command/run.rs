use super::{Command, Validators};
use crate::YgScenarioResult;

pub(crate) fn validate_run(command: &Command) -> YgScenarioResult<()> {
    command.has_no_back_reference()?.has_args()?;

    Ok(())
}

pub(crate) fn validate_conditional(command: &Command) -> YgScenarioResult<()> {
    command.has_args()?;

    Ok(())
}

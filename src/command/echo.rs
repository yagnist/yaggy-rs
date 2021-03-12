use super::{Command, Validators};
use crate::YgScenarioResult;

pub(crate) fn validate(command: &Command) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?.has_args()?;

    Ok(())
}

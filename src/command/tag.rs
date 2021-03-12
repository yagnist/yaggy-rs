use super::{Command, Validators};
use crate::YgScenarioResult;

pub(crate) fn validate_tag(command: &Command) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?.has_args()?;

    // TODO
    // validate tag

    Ok(())
}

pub(crate) fn validate_untag(command: &Command) -> YgScenarioResult<()> {
    command.has_no_reference()?.has_no_back_reference()?.has_args()?;

    // TODO
    // validate untag

    Ok(())
}

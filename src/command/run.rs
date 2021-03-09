use super::{validators, Command};
use crate::YgScenarioResult;

pub(crate) fn validate_run(command: &Command) -> YgScenarioResult<()> {
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    Ok(())
}

pub(crate) fn validate_conditional(command: &Command) -> YgScenarioResult<()> {
    validators::has_args(&command)?;

    Ok(())
}

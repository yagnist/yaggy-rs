use super::{validators, Command};
use crate::YgScenarioResult;

pub(crate) fn validate(command: &Command) -> YgScenarioResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    Ok(())
}

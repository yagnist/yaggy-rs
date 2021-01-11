
use crate::YgResult;
use super::{Command, validators};


pub(crate) fn validate_run(command: &Command) -> YgResult<()> {
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    Ok(())
}

pub(crate) fn validate_conditional(command: &Command) -> YgResult<()> {
    validators::has_args(&command)?;

    Ok(())
}

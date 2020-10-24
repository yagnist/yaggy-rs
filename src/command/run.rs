
use crate::Result;
use super::{Command, validators};


pub(crate) fn validate_run(command: &Command) -> Result<()> {
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    Ok(())
}

pub(crate) fn validate_conditional(command: &Command) -> Result<()> {
    validators::has_args(&command)?;

    Ok(())
}

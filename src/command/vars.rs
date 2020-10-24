
use crate::Result;
use super::{Command, validators};


pub(crate) fn validate(command: &Command) -> Result<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    // TODO
    // validate_args

    Ok(())
}

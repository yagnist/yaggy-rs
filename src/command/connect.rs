
use crate::YgResult;
use super::{Command, validators};


pub(crate) fn validate(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_no_args(&command)?;

    Ok(())
}

pub(crate) fn validate_reconnect(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;

    // TODO
    // validate reconnect

    Ok(())
}

pub(crate) fn validate_reconnect_if(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;

    // TODO
    // validate reconnect_if

    Ok(())
}

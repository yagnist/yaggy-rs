
use crate::Result;
use super::{Command, validators};


pub(crate) fn validate_tag(command: &Command) -> Result<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    // TODO
    // validate tag

    Ok(())
}

pub(crate) fn validate_untag(command: &Command) -> Result<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    // TODO
    // validate untag

    Ok(())
}

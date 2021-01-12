use super::{validators, Command};
use crate::YgResult;

pub(crate) fn validate_tag(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    // TODO
    // validate tag

    Ok(())
}

pub(crate) fn validate_untag(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    // TODO
    // validate untag

    Ok(())
}

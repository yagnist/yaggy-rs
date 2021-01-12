use super::{validators, Command};
use crate::YgResult;

pub(crate) fn validate(command: &Command) -> YgResult<()> {
    validators::has_no_reference(&command)?;
    validators::has_no_back_reference(&command)?;
    validators::has_args(&command)?;

    Ok(())
}

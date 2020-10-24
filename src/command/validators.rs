
use std::rc::Rc;

use crate::{Result, Error};
use super::Command;


pub(crate) fn has_no_reference(command: &Command) -> Result<()> {
    if command.parsed.has_reference() {
        return Err(Error::Syntax { path: Rc::clone(&command.filename), line: command.line_num, message: "Reference is not allowed for this command".to_string() });
    }
    Ok(())
}

pub(crate) fn has_no_back_reference(command: &Command) -> Result<()> {
    if command.parsed.has_back_reference() {
        return Err(Error::Syntax { path: Rc::clone(&command.filename), line: command.line_num, message: "Back reference is not allowed for this command".to_string() });
    }
    Ok(())
}

pub(crate) fn has_args(command: &Command) -> Result<()> {
    if !command.parsed.has_args() {
        return Err(Error::Syntax { path: Rc::clone(&command.filename), line: command.line_num, message: "This command requires some arguments".to_string() });
    }
    Ok(())
}

pub(crate) fn has_no_args(command: &Command) -> Result<()> {
    if command.parsed.has_args() {
        return Err(Error::Syntax { path: Rc::clone(&command.filename), line: command.line_num, message: "This command does not expect any arguments".to_string() });
    }
    Ok(())
}

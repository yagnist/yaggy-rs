use std::error;
use std::fmt;

use super::ParsedLine;
use crate::{YgScenarioError, YgScenarioResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    message: String,
}

impl error::Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub(crate) trait Validators {
    fn has_no_reference(&self) -> ValidationResult<()>;
    fn has_no_back_reference(&self) -> ValidationResult<()>;
    fn has_args(&self) -> ValidationResult<()>;
    fn has_no_args(&self) -> ValidationResult<()>;
}

impl Validators for ParsedLine {
    fn has_no_reference(&self) -> ValidationResult<()> {
        if !self.reference.is_empty() {
            return Err(ValidationError {
                message: "Reference is not allowed for this command"
                    .to_string(),
            });
        }
        Ok(())
    }

    fn has_no_back_reference(&self) -> ValidationResult<()> {
        if !self.back_reference.is_empty() {
            return Err(ValidationError {
                message: "Back reference is not allowed for this command"
                    .to_string(),
            });
        }
        Ok(())
    }

    fn has_args(&self) -> ValidationResult<()> {
        if self.args.is_empty() {
            return Err(ValidationError {
                message: "This command requires some arguments".to_string(),
            });
        }
        Ok(())
    }

    fn has_no_args(&self) -> ValidationResult<()> {
        if !self.args.is_empty() {
            return Err(ValidationError {
                message: "This command does not expect any arguments"
                    .to_string(),
            });
        }
        Ok(())
    }
}

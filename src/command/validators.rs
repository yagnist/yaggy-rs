use super::Command;
use crate::{YgScenarioError, YgScenarioResult};

pub(crate) trait Validators {
    fn has_no_reference(&self) -> YgScenarioResult<&Command>;
    fn has_no_back_reference(&self) -> YgScenarioResult<&Command>;
    fn has_args(&self) -> YgScenarioResult<&Command>;
    fn has_no_args(&self) -> YgScenarioResult<&Command>;
}

impl Validators for Command {
    fn has_no_reference(&self) -> YgScenarioResult<&Command> {
        if self.parsed.has_reference() {
            return Err(YgScenarioError::SyntaxError(
                self.line_num,
                "Reference is not allowed for this command".to_string(),
            ));
        }
        Ok(self)
    }

    fn has_no_back_reference(&self) -> YgScenarioResult<&Command> {
        if self.parsed.has_back_reference() {
            return Err(YgScenarioError::SyntaxError(
                self.line_num,
                "Back reference is not allowed for this command".to_string(),
            ));
        }
        Ok(self)
    }

    fn has_args(&self) -> YgScenarioResult<&Command> {
        if !self.parsed.has_args() {
            return Err(YgScenarioError::SyntaxError(
                self.line_num,
                "This command requires some arguments".to_string(),
            ));
        }
        Ok(self)
    }

    fn has_no_args(&self) -> YgScenarioResult<&Command> {
        if self.parsed.has_args() {
            return Err(YgScenarioError::SyntaxError(
                self.line_num,
                "This command does not expect any arguments".to_string(),
            ));
        }
        Ok(self)
    }
}

use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type ValidationResult<T> = Result<T, String>;

enum State {
    Start,
    Reference,
    AfterReference,
    Command,
    AfterCommand,
    BackReference,
    AfterBackReference,
    Args,
}

#[derive(Debug)]
pub struct ParsedLine {
    pub reference: String,
    pub command: String,
    pub back_reference: String,
    pub args: String,
}

impl FromStr for ParsedLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use State::*;

        let mut chars = s.chars();
        let mut state = Start;
        let mut reference = String::new();
        let mut command = String::new();
        let mut back_reference = String::new();
        let mut args = String::new();

        loop {
            let c = chars.next();
            state = match state {
                Start => match c {
                    None => break,
                    Some('\t') | Some(' ') => Start,
                    Some('@') => {
                        reference.push('@');
                        Reference
                    }
                    Some(c) => {
                        command.push(c);
                        Command
                    }
                },
                Reference => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterReference,
                    Some(c) => {
                        reference.push(c);
                        Reference
                    }
                },
                AfterReference => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterReference,
                    Some(c) => {
                        command.push(c);
                        Command
                    }
                },
                Command => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterCommand,
                    Some(c) => {
                        command.push(c);
                        Command
                    }
                },
                AfterCommand => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterCommand,
                    Some('@') => {
                        back_reference.push('@');
                        BackReference
                    }
                    Some(c) => {
                        args.push(c);
                        Args
                    }
                },
                BackReference => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterBackReference,
                    Some(c) => {
                        back_reference.push(c);
                        BackReference
                    }
                },
                AfterBackReference => match c {
                    None => break,
                    Some('\t') | Some(' ') => AfterBackReference,
                    Some(c) => {
                        args.push(c);
                        Args
                    }
                },
                Args => match c {
                    None => break,
                    Some(c) => {
                        args.push(c);
                        Args
                    }
                },
            }
        }

        if command.is_empty() {
            Err(ParseError { message: "missing command in line".to_string() })
        } else {
            Ok(ParsedLine { reference, command, back_reference, args })
        }
    }
}

impl fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.reference, self.command, self.back_reference, self.args
        )
    }
}

impl ParsedLine {
    pub(super) fn new(
        reference: &str,
        command: &str,
        back_reference: &str,
        args: &str,
    ) -> Self {
        ParsedLine {
            reference: reference.to_string(),
            command: command.to_string(),
            back_reference: back_reference.to_string(),
            args: args.to_string(),
        }
    }
    pub(super) fn has_no_reference(&self) -> ValidationResult<()> {
        if !self.reference.is_empty() {
            return Err(
                "Reference is not allowed for this command".to_string()
            );
        }
        Ok(())
    }

    pub(super) fn has_back_reference(&self) -> ValidationResult<()> {
        if self.back_reference.is_empty() {
            return Err(
                "This command requires back reference to be specified"
                    .to_string(),
            );
        }
        Ok(())
    }
    pub(super) fn has_no_back_reference(&self) -> ValidationResult<()> {
        if !self.back_reference.is_empty() {
            return Err(
                "Back reference is not allowed for this command".to_string()
            );
        }
        Ok(())
    }

    pub(super) fn has_args(&self) -> ValidationResult<()> {
        if self.args.is_empty() {
            return Err("This command requires some arguments".to_string());
        }
        Ok(())
    }

    pub(super) fn has_no_args(&self) -> ValidationResult<()> {
        if !self.args.is_empty() {
            return Err(
                "This command does not expect any arguments".to_string()
            );
        }
        Ok(())
    }
}

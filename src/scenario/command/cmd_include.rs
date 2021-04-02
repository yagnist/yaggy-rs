use std::any::Any;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use super::{from_line, Cmd, ParsedLine};
use crate::{YgPath, YgScenarioError, YgScenarioResult};

#[derive(Debug)]
enum State {
    Valid,
    Invalid(String),
}

#[derive(Debug)]
pub struct CmdInclude {
    filename: Rc<String>,
    line_num: u32,
    to_include: String,
    state: State,
}

impl fmt::Display for CmdInclude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INCLUDE {}", self.to_include)
    }
}

impl CmdInclude {
    // NB. filename    - source file name with INCLUDE directive
    //     line_num    - line number in source file
    //     parsed      - parsed yaggy scenario line
    pub fn new(
        filename: &Rc<String>,
        line_num: u32,
        parsed: &ParsedLine,
    ) -> Self {
        let mut state = State::Valid;
        let basedir = Path::new(filename.as_str()).yg_basedir();
        let to_include = match basedir {
            Ok(basedir) => {
                match basedir.join(&parsed.args).yg_canonicalize() {
                    Ok(to_include) => to_include.to_string_lossy().to_string(),
                    Err(err) => {
                        state = State::Invalid(err.to_string());
                        "".to_string()
                    }
                }
            }
            Err(err) => {
                state = State::Invalid(err.to_string());
                "".to_string()
            }
        };
        CmdInclude {
            filename: Rc::clone(filename),
            line_num: line_num,
            to_include: to_include,
            state: state,
        }
    }
    pub(crate) fn load(&self) -> Vec<YgScenarioResult<Box<dyn Cmd>>> {
        let mut commands: Vec<YgScenarioResult<Box<dyn Cmd>>> = Vec::new();
        if let State::Invalid(ref msg) = self.state {
            commands.push(Err(YgScenarioError::IncludeError(
                Rc::clone(&self.filename),
                self.line_num,
                msg.clone(),
            )));
            return commands;
        }

        let source = match File::open(&self.to_include) {
            Ok(source) => source,
            Err(err) => {
                commands.push(Err(YgScenarioError::OpenError(
                    Rc::clone(&self.filename),
                    err.to_string(),
                )));
                return commands;
            }
        };
        let mut reader = BufReader::new(source);

        let filename = Rc::new(self.to_include.clone());

        // NB. buffer for multiline commands
        let mut buf = String::new();
        let mut line_num = 0;

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    if !buf.is_empty() {
                        commands.push(Err(YgScenarioError::SyntaxError(
                            filename,
                            line_num,
                            "Incomplete multiline command".to_string(),
                        )))
                    }
                    break;
                }
                Ok(_n) => {
                    line_num += 1;
                    let trimmed = line.trim();
                    let is_comment = trimmed.starts_with('#');
                    let is_empty = trimmed.is_empty();
                    let is_multiline = trimmed.ends_with('\\');
                    let trimmed = trimmed.trim_end_matches('\\');

                    if !is_empty && !is_comment {
                        buf.push_str(trimmed);

                        if !is_multiline {
                            let cmd_line = buf.clone();
                            let cmd_line = cmd_line.as_str();
                            commands.push(from_line(
                                &filename, line_num, cmd_line,
                            ));
                            buf.clear();
                        }
                    }
                }
                Err(err) => {
                    commands.push(Err(YgScenarioError::ReadError(
                        filename,
                        err.to_string(),
                    )));
                    break;
                }
            }
        }

        commands
    }
}

impl Cmd for CmdInclude {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

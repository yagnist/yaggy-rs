mod line;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{Command, CommandBuilder, YgScenarioError, YgScenarioResult};
pub(crate) use line::{ParsedLine, ParsedResult};

#[derive(Debug, Default)]
pub(crate) struct Scenario {
    filename: String,
}

impl Scenario {
    pub(crate) fn new(filename: &str) -> Self {
        Scenario { filename: filename.to_string() }
    }
    pub(crate) fn commands(&self) -> YgScenarioResult<ScenarioCommands> {
        let source = File::open(self.filename.clone())
            .map_err(|err| YgScenarioError::OpenError(err))?;
        let reader = BufReader::new(source);
        let commands = ScenarioCommands {
            filename: self.filename.clone(),
            reader: reader,
            line_num: 0,
        };

        Ok(commands)
    }
}

#[derive(Debug)]
pub(crate) struct ScenarioCommands {
    filename: String,
    reader: BufReader<File>,
    line_num: u32,
}

impl Iterator for ScenarioCommands {
    type Item = YgScenarioResult<Command>;

    fn next(&mut self) -> Option<Self::Item> {
        // NB. buffer for multiline commands
        let mut buf = String::new();

        loop {
            // NB. buffer to read single line
            let mut line = String::new();

            match self.reader.read_line(&mut line) {
                Ok(0) => {
                    if !buf.is_empty() {
                        break Some(Err(YgScenarioError::SyntaxError(
                            self.line_num,
                            "Incomplete multiline command".to_string(),
                        )));
                    } else {
                        break None;
                    }
                }
                Ok(_n) => {
                    self.line_num += 1;
                    let trimmed = line.trim();
                    let is_comment = trimmed.starts_with('#');
                    let is_empty = trimmed.is_empty();
                    let is_multiline = trimmed.ends_with('\\');
                    let trimmed = trimmed.trim_end_matches('\\');

                    if !is_empty && !is_comment {
                        buf.push_str(trimmed);

                        if !is_multiline {
                            let parsed: ParsedResult = buf.parse();
                            match parsed {
                                Ok(x) => {
                                    let cmd = CommandBuilder::from_parsed_line(
                                        x,
                                        self.filename.as_str(),
                                        self.line_num,
                                    );
                                    break Some(cmd);
                                }
                                Err(x) => {
                                    break Some(Err(
                                        YgScenarioError::SyntaxError(
                                            self.line_num,
                                            x.to_string(),
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                }
                Err(err) => break Some(Err(YgScenarioError::ReadError(err))),
            }
        }
    }
}

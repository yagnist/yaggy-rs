
mod line;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::{YgResult, YgError, Command, CommandBuilder};
pub(crate) use line::{ParsedLine, ParsedResult};


#[derive(Debug, Default)]
pub(crate) struct Scenario {
    filename: String,
}


impl Scenario {
    pub(crate) fn new(filename: &str) -> Self {
        Scenario {
            filename: filename.to_string(),
        }
    }
    pub(crate) fn lines(&self) -> YgResult<ScenarioLines> {
        let source = File::open(self.filename.clone())
            .map_err(|e| {
                YgError::io_error_with_source(
                    format!(
                        "Error opening scenario \"{}\"",
                        self.filename
                    ),
                    e,
                )
            })?;
        let reader = BufReader::new(source);
        let lines = ScenarioLines { filename: self.filename.clone(), reader: reader, line_num: 0};

        Ok(lines)
    }
    pub(crate) fn commands(&self) -> YgResult<ScenarioCommands> {
        let lines = self.lines()?;
        Ok(ScenarioCommands { lines , included: Box::new(None)})
    }

}


#[derive(Debug)]
pub(crate) struct ScenarioLines {
    filename: String,
    reader: BufReader<File>,
    line_num: u32,
}


impl Iterator for ScenarioLines {
    type Item = YgResult<ParsedLine>;

    fn next(&mut self) -> Option<Self::Item> {
        // NB. buffer for multiline commands
        let mut buf = String::new();

        loop {
            // NB. buffer to read single line
            let mut line = String::new();

            match self.reader.read_line(&mut line) {
                Ok(0) => if !buf.is_empty() {
                    break Some(Err(
                            YgError::scenario_syntax_error(
                                self.filename.clone(),
                                self.line_num,
                                "Incomplete multiline command".to_string(),
                                None,  // FIXME
                            )
                        ))
                } else {
                    break None
                },
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
                                Ok(x) => break Some(Ok(x)),
                                Err(x) => break Some(Err(
                                    YgError::scenario_syntax_error(
                                        self.filename.clone(),
                                        self.line_num,
                                        x.to_string(),
                                        None,  // FIXME
                                    )
                                ))
                            }
                        }
                    }
                },
                Err(e) => break Some(Err(
                    YgError::io_error_with_source(
                        format!(
                            "Error reading scenario \"{}\"",
                            self.filename
                        ),
                        e,
                    )
                ))
            }
        }

    }
}


#[derive(Debug)]
pub(crate) struct ScenarioCommands {
    lines: ScenarioLines,
    included: Box<Option<ScenarioCommands>>,
}

impl Iterator for ScenarioCommands {
    type Item = YgResult<Command>;

    fn next(&mut self) -> Option<Self::Item> {

        // NB. commands from included scenario
        if let Some(ref mut included) = *self.included {
            match included.next() {
                item @ Some(_) => return item,
                None => self.included = Box::new(None),
            }
        }

        // NB. scenario own commands
        let parsed = self.lines.next();

        match parsed {
            Some(Ok(parsed)) => {
                let command = parsed.command.clone();
                if command.as_str() == "INCLUDE" {
                    let included = self.include(&parsed);
                    match included {
                        Ok(included) => {
                            self.included = Box::new(Some(included));
                        },
                        Err(included) => {
                            return Some(Err(included));
                        }
                    }
                }
                let cmd = CommandBuilder::from_parsed_line(
                    parsed,
                    self.lines.filename.as_str(),
                    self.lines.line_num
                );
                Some(cmd)
            },
            Some(Err(parsed)) => Some(Err(parsed)),
            None => None,
        }

    }
}

impl ScenarioCommands {
    fn include(&self, parsed: &ParsedLine) -> YgResult<ScenarioCommands> {
        let to_include = parsed.args.clone();
        let path = Path::new(self.lines.filename.as_str())
            .parent()
            .ok_or(
                YgError::scenario_command_error(
                    self.lines.filename.clone(),
                    self.lines.line_num,
                    "INCLUDE error".to_string(),
                    None,  // FIXME
                )
            )?;
        let filename = path.join(to_include.as_str());
        let filename = filename
            .to_str()
            .ok_or(
                YgError::io_error(
                    format!(
                        "Invalid UTF-8 in \"{}\"",
                        to_include
                    ),
                )
            )?;

        let scenario = Scenario::new(filename);
        scenario.commands()
    }
}

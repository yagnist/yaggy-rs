
mod line;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use crate::{Result, Error, Command, CommandBuilder};
pub(crate) use line::{ParsedLine, ParsedResult};


#[derive(Debug, Default)]
pub(crate) struct Scenario {
    filename: Rc<String>,
}


impl Scenario {
    pub(crate) fn new(filename: String) -> Self {
        Scenario {
            filename: Rc::new(filename),
        }
    }
    pub(crate) fn lines(&self) -> Result<ScenarioLines> {
        let source = File::open(self.filename.as_str())
            .map_err(|e| Error::ScenarioOpen { path: Rc::clone(&self.filename), source: e })?;
        let reader = BufReader::new(source);
        let lines = ScenarioLines { filename: Rc::clone(&self.filename), reader: reader, line_num: 0};

        Ok(lines)
    }
    pub(crate) fn commands(&self) -> Result<ScenarioCommands> {
        let lines = self.lines()?;
        Ok(ScenarioCommands { lines , included: None})
    }

}


#[derive(Debug)]
pub(crate) struct ScenarioLines {
    filename: Rc<String>,
    reader: BufReader<File>,
    line_num: u32,
}


impl Iterator for ScenarioLines {
    type Item = Result<ParsedLine>;

    fn next(&mut self) -> Option<Self::Item> {
        // NB. buffer for multiline commands
        let mut buf = String::new();

        loop {
            // NB. buffer to read single line
            let mut line = String::new();

            match self.reader.read_line(&mut line) {
                Ok(0) => if !buf.is_empty() {
                    break Some(Err(Error::Syntax { path: Rc::clone(&self.filename), line: self.line_num, message: "Incomplete multiline command".to_string() }))
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
                                Err(x) => break Some(Err(Error::Syntax { path: Rc::clone(&self.filename), line: self.line_num, message: x.to_string() }))
                            }
                        }
                    }
                },
                Err(e) => break Some(Err(Error::ScenarioRead { path: Rc::clone(&self.filename), source: e }))
            }
        }

    }
}


#[derive(Debug)]
pub(crate) struct ScenarioCommands {
    lines: ScenarioLines,
    included: Option<Box<ScenarioCommands>>,
}

impl Iterator for ScenarioCommands {
    type Item = Result<Box<dyn Command>>;

    fn next(&mut self) -> Option<Self::Item> {

        // NB. commands from included scenario
        if let Some(ref mut included) = self.included {
            match included.next() {
                item @ Some(_) => return item,
                None => self.included = None,
            }
        }

        // NB. scenario own commands
        let parsed = self.lines.next();

        match parsed {
            Some(Ok(parsed)) => {
                let command = parsed.command.clone();
                if command.as_str() == "INCLUDE" {
                    match self.include(&parsed) {
                        Ok(included) => {
                            self.included = Some(Box::new(included));
                        },
                        Err(included) => {
                            return Some(Err(included));
                        }
                    }
                }
                let cmd = CommandBuilder::from_parsed_line(parsed, &self.lines.filename, self.lines.line_num);
                Some(cmd)
            },
            Some(Err(parsed)) => Some(Err(parsed)),
            None => None,
        }

    }
}

impl ScenarioCommands {
    fn include(&self, parsed: &ParsedLine) -> Result<ScenarioCommands> {
        let to_include = parsed.args.clone();
        let path = Path::new(self.lines.filename.as_str())
            .parent()
            .ok_or(Error::Command { path: Rc::clone(&self.lines.filename), line: self.lines.line_num, message: "INCLUDE error".to_string() })?;
        let filename = path.join(to_include.as_str());
        let filename = filename
            .to_str()
            .ok_or(Error::ScenarioFilename { path: Rc::new(to_include) })?;

        let scenario = Scenario::new(filename.to_string());
        scenario.commands()
    }
}

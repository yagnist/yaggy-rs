
mod line;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{Result, Error};
use line::{ParsedLine, ParsedResult};


#[derive(Debug, Default)]
pub(crate) struct Scenario {
    filename: String,
}


impl Scenario {
    pub(crate) fn new(filename: String) -> Self {
        Scenario {
            filename: filename,
        }
    }
    pub(crate) fn lines(&self) -> Result<ScenarioLines> {
        let source = File::open(&self.filename)
            .map_err(|e| Error::ScenarioOpen { path: self.filename.clone(), source: e })?;
        let reader = BufReader::new(source);
        let lines = ScenarioLines { filename: self.filename.clone(), reader: reader, line_num: 0};

        Ok(lines)
    }

}


#[derive(Debug)]
pub(crate) struct ScenarioLines {
    filename: String,
    reader: BufReader<File>,
    line_num: u32,
}


impl Iterator for ScenarioLines {
    type Item = Result<ParsedLine>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        let mut buf = String::new();

        loop {
            match self.reader.read_line(&mut line) {
                Ok(0) => if !buf.is_empty() {
                    break Some(Err(Error::Syntax { path: self.filename.clone(), line: self.line_num, message: "Incomplete multiline command".to_string() }))
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

                    buf.push_str(trimmed);

                    if !is_empty && !is_comment {
                        if !is_multiline {
                            let parsed: ParsedResult = buf.parse();
                            match parsed {
                                Ok(x) => break Some(Ok(x)),
                                Err(x) => break Some(Err(Error::Syntax { path: self.filename.clone(), line: self.line_num, message: x.to_string() }))
                            }
                        } else {
                            line.clear();
                        }
                    }
                },
                Err(e) => break Some(Err(Error::ScenarioRead { path: self.filename.clone(), source: e }))
            }
        }

    }
}

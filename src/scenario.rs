
use std::path::PathBuf;

#[derive(Debug, Default)]
pub(crate) struct Scenario {
    filename: PathBuf,
}


impl Scenario {
    pub(crate) fn new(filename: PathBuf) -> Self {
        Scenario {
            filename: filename,
        }
    }
}

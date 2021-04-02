mod command;
pub(crate) mod error;

use std::vec::Vec;

use log::info;

use crate::{YgError, YgResult};
use command::{load_main, Cmd};
use error::YgScenarioResult;

pub(crate) struct Scenario {
    commands: Vec<YgScenarioResult<Box<dyn Cmd>>>,
}

impl Scenario {
    pub(crate) fn new(filename: &str) -> Self {
        Scenario { commands: load_main(filename) }
    }
    pub(crate) fn validate(&self) -> YgResult<()> {
        for command in self.commands.iter() {
            match command {
                Ok(command) => {
                    info!("{}", command.to_string());
                }
                Err(err) => {
                    return Err(YgError::Scenario(err.clone()));
                }
            }
        }
        Ok(())
    }
}

mod cli;
mod command;
mod runner;
mod scenario;
mod yg_error;
mod yg_path;

pub(crate) use command::{validate_command, Command, CommandBuilder};
pub(crate) use scenario::{ParsedLine, Scenario};
pub(crate) use yg_error::{
    YgError, YgIoError, YgIoResult, YgResult, YgScenarioError,
    YgScenarioResult,
};
pub(crate) use yg_path::YgPath;

use runner::Runner;

fn main() {
    let cli_args = cli::cli();

    match cli_args.subcommand() {
        ("tags", Some(_tags_args)) => {}
        ("run", Some(run_args)) => {
            let runner = Runner::from_args(&run_args);
            match runner.run() {
                Ok(_x) => println!("[ok]"),
                Err(x) => {
                    eprintln!("{}", x);
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    };
}

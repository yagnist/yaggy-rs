
mod cli;
mod errors;
mod runner;
mod scenario;

pub(crate) use errors::{Result, YaggyError};
pub(crate) use scenario::Scenario;

use runner::Runner;


fn main() {
    let cli_args = cli::cli();

    match cli_args.subcommand() {
        ("tags", Some(_tags_args)) => {},
        ("run", Some(run_args)) => {
            let runner = Runner::from_args(&run_args);
            match runner.run() {
                Ok(_x) => println!("[ok]"),
                Err(x) => {
                    eprintln!("{}", x);
                    std::process::exit(1);
                }
            }
        },
        _ => unreachable!()
    };
}

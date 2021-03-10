mod logging;
mod remote_params;

use std::path::Path;

use clap::ArgMatches;

use log::{debug, error, info, trace, warn};

use crate::{
    validate_command, Scenario, YgError, YgIoError, YgPath, YgResult,
    YgScenarioError,
};
use remote_params::RemoteParams;

#[derive(Debug)]
enum Mode {
    DryRun,
    LiveRun,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::LiveRun
    }
}

#[derive(Debug, Default)]
pub(crate) struct Runner<'a> {
    verbosity: u8,
    mode: Mode,
    filename: &'a str,
    logdir: String,
    runtimedir: String,
    remote_params: RemoteParams,
}

// private methods
impl<'a> Runner<'a> {
    fn new() -> Self {
        Runner { ..Default::default() }
    }
    fn with_verbosity(mut self, verbosity: u8) -> Self {
        self.verbosity = verbosity;
        self
    }
    fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
    fn with_filename(mut self, filename: &'a str) -> Self {
        self.filename = filename;
        self
    }
    fn with_logdir(mut self, logdir: String) -> Self {
        self.logdir = logdir;
        self
    }
    fn with_runtimedir(mut self, runtimedir: String) -> Self {
        self.runtimedir = runtimedir;
        self
    }
    fn with_remote_params(mut self, rparams: RemoteParams) -> Self {
        self.remote_params = rparams;
        self
    }
}

// public methods
impl<'a> Runner<'a> {
    pub(crate) fn from_args(args: &'a ArgMatches) -> Self {
        let verbosity = args.occurrences_of("verbose") as u8;
        let mode = if args.is_present("dry_run") {
            Mode::DryRun
        } else {
            Mode::LiveRun
        };
        let filename = args.value_of("filename").unwrap_or("scenario.yg");
        let logdir = args.value_of("logdir").unwrap_or("logs").to_string();
        let runtimedir =
            args.value_of("runtimedir").unwrap_or(".rt").to_string();

        let rparams = RemoteParams::from_args(args);

        Runner::new()
            .with_verbosity(verbosity)
            .with_mode(mode)
            .with_filename(filename)
            .with_logdir(logdir)
            .with_runtimedir(runtimedir)
            .with_remote_params(rparams)
    }
    pub(crate) fn run(&self) -> YgResult<()> {
        let path = Path::new(self.filename).yg_canonicalize()?;
        let basedir = path.yg_basedir()?;

        let filename_str = path.to_str().ok_or_else(|| {
            YgIoError::UnicodeError(self.filename.to_string())
        })?;

        let logdir = basedir.join(self.logdir.as_str());
        let logdir = logdir
            .as_path()
            .yg_ensure_dir_exists("Logdir")?
            .yg_ensure_is_writable("Logdir")?;

        let logfile = logdir.join(format!(
            "{}.{}.{}.log",
            path.file_stem().unwrap().to_str().unwrap_or("undef"),
            self.remote_params.hostname,
            chrono::Local::now().format("%Y%m%d%H%M%S")
        ));
        logging::setup_logging(self.verbosity, logfile.as_path())?;

        let runtimedir = basedir.join(self.runtimedir.as_str());
        let _runtimedir = runtimedir
            .as_path()
            .yg_ensure_dir_exists("Runtimedir")?
            .yg_ensure_is_writable("Runtimedir")?;

        info!("staring...");
        trace!("trace output...");
        debug!("debugging...");
        warn!("here goes some warning...");
        error!("ooops, something bad happened...");

        run_included(filename_str)
    }
}

fn run_included(filename: &str) -> YgResult<()> {
    let scenario_err = |err| YgError::Scenario(filename.to_string(), err);

    let scenario = Scenario::new(filename);
    let commands = scenario.commands().map_err(scenario_err)?;
    let basedir = Path::new(filename).yg_basedir()?;

    for cmd in commands {
        let cmd = cmd.map_err(scenario_err)?;

        validate_command(basedir, &cmd).map_err(scenario_err)?;
        // print!("{:?}", cmd);
        info!("{}", cmd);

        if cmd.is_include() {
            let path = Path::new(filename).parent().ok_or_else(|| {
                YgError::Scenario(
                    filename.to_string(),
                    YgScenarioError::IncludeError(cmd.line_num),
                )
            })?;
            let to_include = path.join(cmd.parsed.args.as_str());
            let to_include = to_include.to_str().ok_or_else(|| {
                YgError::Scenario(
                    filename.to_string(),
                    YgScenarioError::UnicodeError(
                        cmd.line_num,
                        cmd.parsed.args,
                    ),
                )
            })?;
            run_included(to_include)?;
        }
    }

    Ok(())
}

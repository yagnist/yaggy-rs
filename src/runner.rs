
mod logging;
mod remote_params;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use clap::ArgMatches;

use log::{trace, debug, info, warn, error};

use crate::{Scenario, Result, YaggyError};
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
pub(crate) struct Runner {
    verbosity: u8,
    mode: Mode,
    filename: String,
    logdir: String,
    runtimedir: String,
    remote_params: RemoteParams,
}


impl Runner {
    pub(crate) fn from_args(args: &ArgMatches) -> Self {
        let verbosity = args.occurrences_of("verbose") as u8;
        let mode = if args.is_present("dry_run") { Mode::DryRun } else { Mode::LiveRun };
        let filename = args.value_of("filename").unwrap_or("scenario.yg").to_string();
        let logdir = args.value_of("logdir").unwrap_or("logs").to_string();
        let runtimedir = args.value_of("runtimedir").unwrap_or(".rt").to_string();

        let rparams = RemoteParams::from_args(args);

        Runner::new()
            .with_verbosity(verbosity)
            .with_mode(mode)
            .with_filename(filename)
            .with_logdir(logdir)
            .with_runtimedir(runtimedir)
            .with_remote_params(rparams)
    }
    pub(crate) fn run(&self) -> Result<()> {
        let filename = Path::new(self.filename.as_str()).canonicalize()?;
        let basedir = filename.parent().ok_or(YaggyError::UnknownBasedir)?;

        let logdir = basedir.join(self.logdir.as_str());
        ensure_dir_exists(logdir.as_path())
            .map_err(|x| {
                eprintln!("Error ensuring log directory is available, path: \"{}\"", logdir.as_path().display());
                x
            })?;

        let runtimedir = basedir.join(self.runtimedir.as_str());
        ensure_dir_exists(runtimedir.as_path())
            .map_err(|x| {
                eprintln!("Error ensuring runtime directory is available, path: \"{}\"", runtimedir.as_path().display());
                x
            })?;

        let logfile = logdir.join(
            format!("{}.{}.{}.log",
                filename.file_stem().unwrap().to_str().unwrap_or("undef"),
                self.remote_params.hostname,
                chrono::Local::now().format("%Y%m%d%H%M%S")
            ));
        logging::setup_logging(self.verbosity, logfile.as_path())?;

        let _scenario = Scenario::new(filename);


        info!("staring...");
        trace!("trace output...");
        debug!("debugging...");
        warn!("here goes some warning...");
        error!("ooops, something bad happened...");

        Ok(())
    }
    fn new() -> Self {
        Runner {
            ..Default::default()
        }
    }
    fn with_verbosity(mut self, verbosity: u8) -> Self {
        self.verbosity = verbosity;
        self
    }
    fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
    fn with_filename(mut self, filename: String) -> Self {
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

fn ensure_dir_exists(path: &Path) -> Result<()> {
    if path.is_dir() {
        return Ok(());
    }
    let perms = fs::Permissions::from_mode(0o700);

    fs::create_dir(&path)?;
    fs::set_permissions(&path, perms)?;

    Ok(())
}

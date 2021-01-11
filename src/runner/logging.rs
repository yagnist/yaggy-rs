
use std::io;
use std::path::Path;

use crate::YgResult;


pub(crate) fn setup_logging(verbosity: u8, logfile: &Path) -> YgResult<()> {

    let level = match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    let base_config = fern::Dispatch::new().level(level);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                    "{:.1} {} | {} | {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S"),
                    record.target(),
                    message
            ))
        })
        .chain(io::stdout());

    let logfile_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                    "{:.1} {} | {} | {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S"),
                    record.target(),
                    message
            ))
        })
        .chain(fern::log_file(logfile)?);

    base_config
        .chain(stdout_config)
        .chain(logfile_config)
        .apply()?;

    Ok(())
}

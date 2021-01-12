use std::path::Path;

use clap::{
    crate_description, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};

fn is_int(v: String) -> Result<(), String> {
    match v.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("value must be valid positive integer")),
    }
}

fn path_exists(p: String) -> Result<(), String> {
    match Path::new(p.as_str()).exists() {
        true => Ok(()),
        false => Err(String::from("path does not exist or is inaccessible")),
    }
}

pub(super) fn cli() -> ArgMatches<'static> {
    App::new("yaggy")
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpFlags)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommands(vec![
            SubCommand::with_name("run")
                .about("Run yaggy scenario")
                .arg(Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .display_order(2)
                    .validator(is_int)
                    .help("Remote port to connect to (optional)"))
                .arg(Arg::with_name("user")
                    .short("u")
                    .long("user")
                    .takes_value(true)
                    .display_order(3)
                    .help("Remote host user to connect as (optional)"))
                .arg(Arg::with_name("tags")
                    .short("t")
                    .long("tags")
                    .takes_value(true)
                    .use_delimiter(true)
                    .require_delimiter(true)
                    .display_order(4)
                    .help("Comma-separated list of tags to run actions for (optional)"))
                .arg(Arg::with_name("syncroot")
                    .short("s")
                    .long("syncroot")
                    .takes_value(true)
                    .default_value("~/.yaggy")
                    .display_order(5)
                    .help("Remote server directory to copy files and render templates to"))
                .arg(Arg::with_name("logdir")
                    .short("l")
                    .long("logdir")
                    .takes_value(true)
                    .default_value("logs")
                    .help("Local directory to store yaggy logs"))
                .arg(Arg::with_name("runtimedir")
                    .short("r")
                    .long("runtimedir")
                    .takes_value(true)
                    .default_value(".rt")
                    .help("Local runtime directory to store ssh control path socket file"))
                .arg(Arg::with_name("dry_run")
                    .long("dry-run")
                    .help("Dry-run mode to test connection and validate scenario syntax"))
                .arg(Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .help("Increases logging verbosity (specify twice for maximum verbosity)"))
                .arg(Arg::with_name("host")
                    .required(true)
                    .index(1)
                    .help("Remote host to connect to"))
                .arg(Arg::with_name("filename")
                     .required(true)
                     .index(2)
                     .validator(path_exists)
                     .help("Yaggy scenario to execute on the remote host")),
            SubCommand::with_name("tags")
                .about("Show tags tree built from yaggy scenario")
                .arg(Arg::with_name("filename")
                     .help("yaggy scenario (required)")
                     .required(true)),
        ])
        .get_matches()
}

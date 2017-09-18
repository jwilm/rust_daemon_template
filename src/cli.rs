use std::path::PathBuf;

use clap::{Arg, App};
use log;

#[derive(Debug)]
pub struct Options {
    /// Path to the configuration file to use
    pub config_path: PathBuf,

    /// The max level of logging
    pub max_log_level: log::LogLevelFilter,
}

impl Options {
    pub fn load() -> Options {
        let matches = App::new("rust_daemon_template")
            .version("0.0.1")
            .author("Joe Wilm <joe@jwilm.com>")
            .about("Description of rust_daemon_template goes here")
            // -c, --config
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Increases verbosity; may be specified up to three times"))
            .get_matches();

        // Gets a value for config if supplied by user, or provides default.
        let config_path = PathBuf::from(matches.value_of("config")
            .unwrap_or("/etc/rust_daemon_template/config.yml"));

        let level = match matches.occurrences_of("v") {
            0 => log::LogLevelFilter::Warn,
            1 => log::LogLevelFilter::Info,
            2 => log::LogLevelFilter::Debug,
            _ => log::LogLevelFilter::Trace
        };

        Options {
            config_path: config_path,
            max_log_level: level,
        }
    }
}

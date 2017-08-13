//! rust_daemon_template description

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_yaml;

use std::process::exit;

mod config;
mod cli;
mod logging;

use config::Config;

/// The main function
///
/// Delegates to `run()` to provide error handling.
fn main() {
    if let Err(err) = run() {
        error!("rust_daemon_template encountered a fatal error: {}", err);
        exit(1);
    }
}

fn run() -> ::std::result::Result<(), Box<::std::error::Error>> {
    // Load command-line options
    let opts = cli::Options::load();

    // Initialize logging
    let _ = logging::init(&opts);

    // Load configuration
    let _config = Config::load(&opts)?;

    // Program logic goes here
    Ok(())
}

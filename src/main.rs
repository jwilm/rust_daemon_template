//! rust_daemon_template description

#[macro_use] extern crate chan;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate chan_signal;
extern crate clap;
extern crate serde;
extern crate serde_yaml;

use std::process::exit;

mod application; // general app stuff
mod app; // app stuff for *this* program
mod config;
mod cli;
mod logging;

use config::Config;
use app::App;
use application::Application;

/// The main function
///
/// Delegates to `run()` to provide error handling.
fn main() {
    if let Err(err) = run::<App>() {
        error!("rust_daemon_template encountered a fatal error: {}", err);
        exit(1);
    }
}

fn run<T: Application>() -> Result<(), Box<::std::error::Error>> {
    // Setup signal handling. This must be called before spawning any other
    // threads so that all threads inherit the blocked status of these named
    // signals.
    let signal = chan_signal::notify(T::signals());

    // Load command-line options
    let opts = cli::Options::load();

    // Initialize logging
    let _ = logging::init(&opts);

    // Load configuration
    let config = Config::load(&opts)?;

    // Initialize the application.
    application::run::<T>(opts, config, signal)?;

    // Program logic goes here
    Ok(())
}

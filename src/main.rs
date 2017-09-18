//! rust_daemon_template description

#[macro_use] extern crate chan;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate chan_signal;
extern crate clap;
extern crate serde;
extern crate serde_yaml;

use chan::Receiver;
use chan_signal::Signal;

use std::process::exit;

mod config;
mod cli;
mod logging;

use config::Config;

struct Runner;

impl Runner {
    fn run<T>(
        opts: cli::Options,
        config: Config,
        signal: Receiver<Signal>
    ) -> ::std::result::Result<(), <T as Application>::Err>
        where T: Application
    {
        let mut app = T::new(opts, config)?;

        loop {
            if let Stopping::Yes = app.run_once()? {
                break;
            }

            chan_select! {
                default => {},
                signal.recv() -> sig => {
                    debug!("Received signal {:?}", sig);
                    sig.map(|s| app.received_signal(s));
                },
            }
        }

        app.shutdown()?;
        Ok(())
    }
}

struct App;

#[derive(Debug)]
enum Error {
    Other(Box<::std::error::Error>),
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        use self::Error::*;
        match *self {
            Other(ref err) => Some(&**err),
        }
    }

    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Other(ref err) => err.description(),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Error::*;
        match *self {
            Other(ref err) => ::std::fmt::Display::fmt(err, f)
        }
    }
}

impl Application for App {
    type Err = Error;

    fn new(_: cli::Options, _: Config) -> Result<Self, Self::Err> {
        Ok(App)
    }

    fn run_once(&mut self) -> Result<Stopping, Self::Err> {
        Ok(Stopping::Yes)
    }
}

enum Stopping {
    Yes,
    No
}

trait Application: Sized {
    type Err: ::std::error::Error + 'static;

    fn new(opts: cli::Options, config: Config) -> Result<Self, Self::Err>;

    /// Called repeatedly in the main loop of the application.
    fn run_once(&mut self) -> Result<Stopping, Self::Err>;

    /// Which signal the application is interested in receiving.
    ///
    /// By default, only INT and TERM are blocked and handled.
    fn signals() -> &'static [Signal] {
        static SIGNALS: &[Signal] = &[Signal::INT, Signal::TERM];
        SIGNALS
    }

    /// Handle a received signal
    fn received_signal(&mut self, signal: Signal) -> Result<Stopping, Self::Err> {
        Ok(Stopping::Yes)
    }

    /// Called when the application is shutting down
    fn shutdown(self) -> Result<(), Self::Err> {
        Ok(())
    }
}

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
    Runner::run::<T>(opts, config, signal)?;

    // Program logic goes here
    Ok(())
}

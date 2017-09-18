use std::fmt;

use application::{Application, Stopping};
use cli::Options;
use config::Config;

/// Core program logic
///
/// Must implement the `Application` trait.
pub struct App;

/// Error type for the core program logic
#[derive(Debug)]
pub enum Error {
    Placeholder
}

impl Application for App {
    type Err = Error;

    fn new(_: Options, _: Config) -> Result<Self, Self::Err> {
        Ok(App)
    }

    fn run_once(&mut self) -> Result<Stopping, Self::Err> {
        Ok(Stopping::Yes)
    }
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        use self::Error::*;
        match *self {
            Placeholder => None,
        }
    }

    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Placeholder => "placeholder error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Placeholder => f.write_str("placeholder"),
        }
    }
}

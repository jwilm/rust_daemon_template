//! General types applicable to any Application
use chan::Receiver;
use chan_signal::Signal;

use cli::Options;
use config::Config;

/// Indicates whether the run loop should halt
pub enum Stopping {
    /// The run loop should halt
    Yes,

    /// The run loop should continue
    No
}

/// The application; domain-specific program logic
pub trait Application: Sized {
    type Err: ::std::error::Error + 'static;

    /// Create a new instance given the options and config
    fn new(_: Options, _: Config) -> Result<Self, Self::Err>;

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
    fn received_signal(&mut self, _: Signal) -> Result<Stopping, Self::Err> {
        Ok(Stopping::Yes)
    }

    /// Called when the application is shutting down
    fn shutdown(self) -> Result<(), Self::Err> {
        Ok(())
    }
}

/// Run an Application of type T
///
/// `run` creates an application from `opts` and `config`. A run loop is entered
/// where `run_once` is repeatedly called on the `T`. Between calls, any
/// arriving signals are checked for and passed to the application via
/// `received_signal`.
pub fn run<T>(
    opts: Options,
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

        // Handle any and all pending signals.
        loop {
            chan_select! {
                default => { break; },
                signal.recv() -> sig => {
                    debug!("Received signal {:?}", sig);
                    sig.map(|s| app.received_signal(s));
                },
            }
        }
    }

    app.shutdown()?;
    Ok(())
}

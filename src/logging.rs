use std::sync;
use std::io;

use log;

use cli::Options;

struct Logger<T> {
    level: log::LogLevelFilter,
    output: sync::Mutex<T>
}

impl<T: Send + io::Write> Logger<T> {
    pub fn new(output: T, level: log::LogLevelFilter) -> Logger<io::LineWriter<T>> {
        Logger {
            level: level,
            output: sync::Mutex::new(io::LineWriter::new(output))
        }
    }
}

impl<T: Send + io::Write> log::Log for Logger<T> {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= self.level &&
            metadata.target().starts_with("rust_daemon_template")
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            if let Ok(ref mut writer) = self.output.lock() {
                let _ = writeln!(writer, "{}", record.args());
            }
        }
    }
}

pub fn init(opts: &Options) -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(opts.max_log_level);
        Box::new(Logger::new(io::stderr(), opts.max_log_level))
    })
}

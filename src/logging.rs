use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use log::{debug, info, warn, error};
use env_logger::{Builder, Target};


static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn logging_tests() {
    log::set_logger(&CONSOLE_LOGGER);
    log::set_max_level(LevelFilter::Info);

    log::debug!("Logging test (debug)"); // Should not be printed
    log::info!("Logging test (info)");
    log::warn!("Logging test (warn)");
    log::error!("Logging test (error)");
}

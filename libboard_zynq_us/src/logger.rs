//! A logger for the `log` crate

use crate::{println, stdio};

pub static LOGGER: Logger = Logger;

pub struct Logger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{:>5}({}): {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }
    fn flush(&self) {
        let uart = stdio::get_uart();
        while !uart.tx_idle() {}
    }
}

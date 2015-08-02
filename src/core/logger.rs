extern crate log;

use self::log::{LogRecord, LogMetadata};

///The Logger used by Netherrack
///
///For now, it simply logs to standard output. In future versions, it will be extended
pub struct NetherrackLogger;

impl log::Log for NetherrackLogger {

    ///Checks to see if a LogRecord can be accepted based on LogMetadata
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        true //Before 0.1, change dynamically based on if we're a prerelease version
    }

    ///Attempts to log a LogRecord
    fn log(&self, record: &LogRecord) {
        //First, check to see if logging is enabled for the metadata
        if self.enabled(record.metadata()) {
            println!("[{}] - {}", record.level(), record.args());
        }
    }
}
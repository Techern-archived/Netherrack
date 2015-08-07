//! Placeholder that enables running Netherrack as a standalone executable.
//!
//! Actual logic is in the library itself so that, for example, an embedded server can be created.
//!
//! All you would need to do is set the logger!

extern crate netherrack;

pub use netherrack::*;

#[macro_use]
extern crate log;

/// Starts a standalone version of Netherrack
///
/// It actually just sets the loggers, and then starts an embedded server :)
fn main() {

    if set_global_logger() {
        info!("Set up logging");
    } else {
        error!("Could not set up logging");
    }

    info!("Attempting to load Netherrack {}", get_version());
    
    start_server();

}

fn set_global_logger() -> bool {

    let log_setup_result: Result<(), log::SetLoggerError> = log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Trace);
        Box::new(core::logger::NetherrackLogger)
    });

    if log_setup_result.is_ok() {
        return true;
    } else {
        return false;
    }

}
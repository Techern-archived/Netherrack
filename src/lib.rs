//! Entry module for the Netherrack project

// We'll be using serialization a lot
extern crate rustc_serialize;

// Enable logging and use of logging macros throughout Netherrack
#[macro_use]
extern crate log;

// Enable global use of lazy_static
#[macro_use]
extern crate lazy_static;

// Allow use of Semantic Versioning througout Netherrack
extern crate semver;
// Also allow usage of Version through Netherrack
pub use semver::Version;

// Allow use of the core module
pub mod core;

// Import I/O operations before Netherrack-specific ones
extern crate io_operations;
// Allow access to I/O operations
pub mod io;

// Allow access to the universe
pub mod universe;


pub use std::sync::mpsc::{ Sender, Receiver, channel };

// Okay, this is the start of Netherrack's functions

/// The version of the Minecraft protocol accepted by Netherrack
pub const MINECRAFT_PROTOCOL_VERSION: u32 = 47;

/// The version of Netherrack as determined at compile time
pub const NETHERRACK_VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");

/// Gets the current version of Netherrack
pub fn get_version() -> Version {
    Version::parse(NETHERRACK_VERSION_STRING).unwrap()
}

/// Starts a new Netherrack server instance
pub fn start_server() {

    info!("Netherrack startup called");

    trace!("Starting network in a new thread");

    std::thread::spawn(move || {
        io::network::start_network();
    });

    debug!("Networking should be set up");

    loop {
        //TODO: In the future, execute a tick. Use Duration's span function to get time taken, sleep the remainder, and go again
        std::thread::sleep_ms(20);
    }

}

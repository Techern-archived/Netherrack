//! Entry module for the Netherrack project

// Enable logging and use of logging macros throughout Netherrack
#[macro_use]
extern crate log;

// Allow use of Semantic Versioning througout Netherrack
extern crate semver;
// Also allow usage of Version through Netherrack
pub use semver::Version;

// Allow use of the core module
pub mod core;

// Allow access to the universe
pub mod universe;

// Okay, this is the start of Netherrack's functions

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
        core::network::start_network();
    });
    
    debug!("Networking should be set up");
    
    loop {
        //TODO: In the future, execute a tick. Use Duration's span function to get time taken, sleep the remainder, and go again
        std::thread::sleep_ms(20);
    }
    
}
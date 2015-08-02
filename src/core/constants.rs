//! Temporary placement of constants in Rusted Minecraft that will later be moved elsewhere

extern crate semver;

pub use self::semver::Version;

/// The version of Rusted Minecraft as determined at compile time
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Gets the current version of Rusted Minecraft
pub fn get_version() -> Version {
    Version::parse(VERSION).unwrap()
}
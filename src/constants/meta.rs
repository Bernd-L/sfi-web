// Not every constant is used
#![allow(dead_code)]

// The name of the application
pub const NAME: &'static str = "sfi-web";

/// The main author of the application
pub const AUTHOR: &'static str = "Bernd-L <git@bernd.pw>";

/// The semantic-version string of the application
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Describes the application (i.e. its use cases) in a short phrase
pub const ABOUT: &'static str = "Manage shared food inventories";

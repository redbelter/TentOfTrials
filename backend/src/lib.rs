pub mod ai;
pub mod config;
pub mod discovery;
pub mod messaging;
pub mod registry;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_PROFILE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

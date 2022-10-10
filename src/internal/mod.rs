#[macro_use]
mod macros;

pub mod config;
pub mod crypto;
pub mod data;
pub mod dirs;
pub mod logging;
pub mod net;

/// Checks if the running app is dockerized
pub fn is_dockerized() -> bool {
    option_env!("DOCKERIZED").is_some()
}

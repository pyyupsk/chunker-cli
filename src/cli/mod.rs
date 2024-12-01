mod commands;
mod config;

pub use commands::{handle_merge, handle_split};
pub use config::build_cli;

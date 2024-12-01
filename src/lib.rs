pub mod cli;
pub mod core;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export commonly used items
pub use core::{merge_files, split_file, ChunkResult};

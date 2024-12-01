use std::fs;
use std::path::{Path, PathBuf};

pub fn cleanup_chunks(chunks: &[PathBuf], dir: &Path) {
    for chunk in chunks {
        if let Err(e) = fs::remove_file(chunk) {
            eprintln!("Warning: Failed to remove chunk {}: {}", chunk.display(), e);
        }
    }
    if let Err(e) = fs::remove_dir(dir) {
        eprintln!("Warning: Failed to remove chunks directory: {}", e);
    } else {
        println!("🧹 Cleaned up chunks directory");
    }
}

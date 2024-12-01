use crate::utils::{cleanup_chunks, parse_size};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_parse_size() {
    assert_eq!(parse_size("1024").unwrap(), 1024.0);
    assert_eq!(parse_size("1KB").unwrap(), 1024.0);
    assert_eq!(parse_size("1MB").unwrap(), 1024.0 * 1024.0);
    assert_eq!(parse_size("1GB").unwrap(), 1024.0 * 1024.0 * 1024.0);
    assert_eq!(
        parse_size("1TB").unwrap(),
        1024.0 * 1024.0 * 1024.0 * 1024.0
    );
    assert_eq!(parse_size("1.5MB").unwrap(), 1.5 * 1024.0 * 1024.0);

    // Test invalid inputs
    assert!(parse_size("invalid").is_err());
    assert!(parse_size("1XB").is_err());
    assert!(parse_size("-1MB").is_err());
}

#[test]
fn test_cleanup_chunks() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    let chunk_paths: Vec<PathBuf> = vec!["chunk1.txt", "chunk2.txt"]
        .into_iter()
        .map(|name| {
            let path = temp_dir.path().join(name);
            fs::File::create(&path).unwrap();
            path
        })
        .collect();

    // Verify files exist
    for path in &chunk_paths {
        assert!(path.exists());
    }

    cleanup_chunks(&chunk_paths, temp_dir.path());

    // Verify files and directory were cleaned up
    for path in &chunk_paths {
        assert!(!path.exists());
    }
    assert!(!temp_dir.path().exists());

    Ok(())
}

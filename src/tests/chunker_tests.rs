use indicatif::ProgressBar;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use crate::chunker;

#[tokio::test]
async fn test_split_and_merge() -> std::io::Result<()> {
    // Create a temporary directory for our test
    let temp_dir = tempdir()?;
    let source_path = temp_dir.path().join("test_file.txt");
    let chunks_dir = temp_dir.path().join("chunks");
    let merged_path = temp_dir.path().join("merged_file.txt");

    // Create test data
    let test_data = "Hello, World!".repeat(1000);
    let mut source_file = fs::File::create(&source_path)?;
    source_file.write_all(test_data.as_bytes())?;

    fs::create_dir(&chunks_dir)?;

    // Test split
    let progress = ProgressBar::new(0);
    let split_result = chunker::split(
        &source_path,
        &chunks_dir,
        2,      // concurrent tasks
        1024.0, // chunk size in bytes
        progress.clone(),
    )
    .await?;

    assert!(split_result.chunks > 0);
    assert!(split_result.time >= 0.0);

    // Verify chunks were created
    let chunks = chunker::get_chunks(&chunks_dir)?;
    assert!(!chunks.is_empty());

    // Test merge
    let merge_time = chunker::merge(
        chunks,
        &merged_path,
        2,      // concurrent tasks
        1024.0, // buffer size
        progress,
    )
    .await?;

    assert!(merge_time >= 0.0);

    // Verify merged content matches original
    let merged_content = fs::read_to_string(&merged_path)?;
    assert_eq!(merged_content, test_data);

    Ok(())
}

#[test]
fn test_get_chunks() -> std::io::Result<()> {
    let temp_dir = tempdir()?;

    // Create some test chunk files
    let files = vec!["file_chunk1.txt", "file_chunk2.txt", "file_chunk10.txt"];
    for file in &files {
        fs::File::create(temp_dir.path().join(file))?;
    }

    let chunks = chunker::get_chunks(temp_dir.path())?;

    assert_eq!(chunks.len(), 3);
    // Verify correct ordering
    assert!(chunks[0].to_str().unwrap().contains("chunk1"));
    assert!(chunks[1].to_str().unwrap().contains("chunk2"));
    assert!(chunks[2].to_str().unwrap().contains("chunk10"));

    Ok(())
}

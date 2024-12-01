use indicatif::ProgressBar;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio;

pub async fn merge_files(
    chunks: Vec<PathBuf>,
    output_path: &Path,
    concurrent: usize,
    buffer_size: f64,
    progress: ProgressBar,
) -> io::Result<f64> {
    let start_time = Instant::now();

    let total_size: u64 = chunks
        .iter()
        .map(|path| fs::metadata(path).map(|m| m.len()).unwrap_or(0))
        .sum();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)?;
    file.set_len(total_size)?;

    progress.set_length(chunks.len() as u64);

    for chunk_group in chunks.chunks(concurrent) {
        let mut tasks = Vec::new();

        for chunk_path in chunk_group {
            let chunk_path = chunk_path.clone();
            let progress = progress.clone();

            tasks.push(tokio::spawn(async move {
                let chunk_size = fs::metadata(&chunk_path)?.len();
                let mut reader =
                    BufReader::with_capacity(buffer_size as usize, File::open(chunk_path)?);
                let mut buffer = vec![0; chunk_size as usize];
                reader.read_exact(&mut buffer)?;

                progress.inc(1);
                Ok::<_, io::Error>(buffer)
            }));
        }

        let mut writer = BufWriter::with_capacity(buffer_size as usize, &file);
        for task in tasks {
            let buffer = task.await.unwrap()?;
            writer.write_all(&buffer)?;
        }
        writer.flush()?;
    }

    progress.finish();
    Ok(start_time.elapsed().as_secs_f64())
}

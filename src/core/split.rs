use indicatif::ProgressBar;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek};
use std::path::Path;
use std::time::Instant;
use tokio;

use crate::ChunkResult;

pub async fn split_file(
    source_file: &Path,
    output_dir: &Path,
    concurrent: usize,
    chunk_size: f64,
    progress: ProgressBar,
) -> io::Result<ChunkResult> {
    let start_time = Instant::now();
    let file = File::open(source_file)?;
    let file_size = file.metadata()?.len();

    let name_base = source_file.file_stem().unwrap().to_str().unwrap();
    let ext = source_file.extension().unwrap().to_str().unwrap();

    let num_chunks = ((file_size as f64) / (chunk_size as f64)).ceil() as usize;
    progress.set_length(num_chunks as u64);

    let mut chunk_files = Vec::with_capacity(num_chunks);
    for i in 0..num_chunks {
        let chunk_name = format!("{}_chunk{}.{}", name_base, i + 1, ext);
        let chunk_path = output_dir.join(chunk_name);
        let chunk_size = if i == num_chunks - 1 {
            file_size - (i as u64 * chunk_size as u64)
        } else {
            chunk_size as u64
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&chunk_path)?;
        file.set_len(chunk_size)?;
        chunk_files.push(chunk_path);
    }

    for i in (0..num_chunks).step_by(concurrent) {
        let mut tasks = Vec::new();

        for j in i..std::cmp::min(i + concurrent, num_chunks) {
            let chunk_index = j;
            let start = chunk_index as u64 * chunk_size as u64;
            let end = std::cmp::min(start + chunk_size as u64, file_size);
            let chunk_name = format!("{}_chunk{}.{}", name_base, chunk_index + 1, ext);
            let chunk_path = output_dir.join(chunk_name);

            let mut buffer = vec![0; (end - start) as usize];
            let mut file = file.try_clone().unwrap();
            file.seek(std::io::SeekFrom::Start(start)).unwrap();
            file.read_exact(&mut buffer).unwrap();

            let progress = progress.clone();
            tasks.push(tokio::spawn(async move {
                fs::write(chunk_path, buffer).unwrap();
                progress.inc(1);
                Ok::<_, io::Error>(())
            }));
        }

        for task in tasks {
            task.await.unwrap().unwrap();
        }
    }

    progress.finish();
    let time = start_time.elapsed().as_secs_f64();

    Ok(ChunkResult {
        chunks: num_chunks,
        time,
    })
}

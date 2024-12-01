use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio;

#[derive(Debug)]
pub struct ChunkResult {
    pub chunks: usize,
    pub time: f64,
}

async fn process_chunk_batch(
    file: &File,
    file_size: u64,
    start_chunk: usize,
    batch_size: usize,
    output_dir: &Path,
    name_base: &str,
    ext: &str,
    progress: &indicatif::ProgressBar,
    chunk_size: usize,
) -> io::Result<()> {
    let mut tasks = Vec::new();

    for i in 0..batch_size {
        let chunk_index = start_chunk + i;
        let start = chunk_index as u64 * chunk_size as u64;
        if start >= file_size {
            break;
        }

        let end = std::cmp::min(start + chunk_size as u64, file_size);
        let chunk_name = format!("{}_chunk{}.{}", name_base, chunk_index + 1, ext);
        let chunk_path = output_dir.join(chunk_name);

        let mut buffer = vec![0; (end - start) as usize];
        let mut file = file.try_clone()?;
        file.seek(std::io::SeekFrom::Start(start))?;
        file.read_exact(&mut buffer)?;

        let progress = progress.clone();
        tasks.push(tokio::spawn(async move {
            fs::write(chunk_path, buffer)?;
            progress.inc(1);
            Ok::<_, io::Error>(())
        }));
    }

    for task in tasks {
        task.await.unwrap()?;
    }

    Ok(())
}

pub async fn split(
    source_file: &Path,
    output_dir: &Path,
    concurrent: usize,
    progress: indicatif::ProgressBar,
    chunk_size: usize,
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
        process_chunk_batch(
            &file, file_size, i, concurrent, output_dir, name_base, ext, &progress, chunk_size,
        )
        .await?;
    }

    progress.finish();
    let time = start_time.elapsed().as_secs_f64();

    Ok(ChunkResult {
        chunks: num_chunks,
        time,
    })
}

pub async fn merge(
    chunks: Vec<PathBuf>,
    output_path: &Path,
    progress: indicatif::ProgressBar,
    buffer_size: usize,
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

    let mut writer = BufWriter::with_capacity(buffer_size, file);
    progress.set_length(chunks.len() as u64);

    for chunk_path in chunks {
        let mut reader = BufReader::with_capacity(buffer_size, File::open(chunk_path)?);
        io::copy(&mut reader, &mut writer)?;
        progress.inc(1);
    }

    writer.flush()?;
    progress.finish();

    Ok(start_time.elapsed().as_secs_f64())
}

pub fn get_chunks(output_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut chunks: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    chunks.sort_by(|a, b| {
        let get_num = |p: &Path| {
            p.file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| {
                    n.chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<usize>()
                        .ok()
                })
                .unwrap_or(0)
        };
        get_num(a).cmp(&get_num(b))
    });

    Ok(chunks)
}

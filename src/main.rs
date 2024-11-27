use clap::{Command, Arg};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter, Seek};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio;

const CHUNK_SIZE: usize = 25_684_992; // 24.5MB in bytes

#[derive(Debug)]
struct ChunkResult {
    chunks: usize,
    time: f64,
}

async fn process_chunk_batch(
    file: &File,
    file_size: u64,
    start_chunk: usize,
    batch_size: usize,
    output_dir: &Path,
    name_base: &str,
    ext: &str,
    progress: &ProgressBar,
) -> io::Result<()> {
    let mut tasks = Vec::new();

    for i in 0..batch_size {
        let chunk_index = start_chunk + i;
        let start = chunk_index as u64 * CHUNK_SIZE as u64;
        if start >= file_size {
            break;
        }

        let end = std::cmp::min(start + CHUNK_SIZE as u64, file_size);
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

fn get_chunks(output_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut chunks: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    chunks.sort_by(|a, b| {
        let get_num = |p: &Path| {
            p.file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().ok())
                .unwrap_or(0)
        };
        get_num(a).cmp(&get_num(b))
    });

    Ok(chunks)
}

async fn split(
    source_file: &Path,
    output_dir: &Path,
    concurrent: usize,
    progress: ProgressBar,
) -> io::Result<ChunkResult> {
    if !source_file.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source file does not exist"));
    }

    let start_time = Instant::now();
    let file = File::open(source_file)?;
    let file_size = file.metadata()?.len();
    
    let name_base = source_file.file_stem().unwrap().to_str().unwrap();
    let ext = source_file.extension().unwrap().to_str().unwrap();

    let num_chunks = ((file_size as f64) / (CHUNK_SIZE as f64)).ceil() as usize;
    progress.set_length(num_chunks as u64);

    for i in (0..num_chunks).step_by(concurrent) {
        process_chunk_batch(
            &file,
            file_size,
            i,
            concurrent,
            output_dir,
            name_base,
            ext,
            &progress,
        ).await?;
    }

    progress.finish();
    let time = start_time.elapsed().as_secs_f64();

    Ok(ChunkResult { chunks: num_chunks, time })
}

async fn merge(chunks: Vec<PathBuf>, output_path: &Path, progress: ProgressBar) -> io::Result<f64> {
    let start_time = Instant::now();
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    progress.set_length(chunks.len() as u64);

    for chunk_path in chunks {
        let mut chunk_file = BufReader::new(File::open(chunk_path)?);
        let mut buffer = Vec::new();
        chunk_file.read_to_end(&mut buffer)?;
        writer.write_all(&buffer)?;
        progress.inc(1);
    }

    writer.flush()?;
    progress.finish();
    
    Ok(start_time.elapsed().as_secs_f64())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Command::new("chunker-cli")
        .about("🚀 CLI tool for splitting and merging large files".cyan().to_string())
        .version("1.0.0")
        .subcommand(
            Command::new("split")
                .about("🔪 Split a file into smaller chunks")
                .arg(Arg::new("source")
                    .help("Source file path".yellow().to_string())
                    .required(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Output directory for chunks (default: source_chunks)".yellow().to_string()))
                .arg(Arg::new("concurrent")
                    .short('c')
                    .long("concurrent")
                    .help("Number of concurrent chunks to process (default: 5)".yellow().to_string())
                    .value_parser(clap::value_parser!(usize)))
        )
        .subcommand(
            Command::new("merge")
                .about("🧩 Merge file chunks back into a single file")
                .arg(Arg::new("directory")
                    .help("Directory containing chunk files".yellow().to_string())
                    .required(true))
                .arg(Arg::new("output")
                    .help("Output file path".yellow().to_string())
                    .required(true))
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => {
            let source = PathBuf::from(sub_matches.get_one::<String>("source").unwrap());
            let output_dir = sub_matches
                .get_one::<String>("output")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(format!("{}_chunks", source.display())));
            let concurrent = sub_matches
                .get_one::<usize>("concurrent")
                .copied()
                .unwrap_or(5);

            fs::create_dir_all(&output_dir)?;

            println!("{}", format!("📂 Preparing to split {}...\n", source.display()).green());

            let progress = ProgressBar::new(0).with_style(
                ProgressStyle::default_bar()
                    .template("📊 {bar:40} | {percent}% | {pos}/{len} chunks")
                    .unwrap()
                    .progress_chars("█░░"),
            );

            match split(&source, &output_dir, concurrent, progress).await {
                Ok(result) => {
                    println!("\n{}\n", "\n✅ Split complete! 🎉".green().bold());
                    println!("  📦 Chunks created: {}", result.chunks);
                    println!("  ⏱️  Time taken: {}s", result.time.round());
                    println!("  📁 Output directory: {}\n", output_dir.display());
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Error splitting file:".red().bold(), e.to_string().red());
                    std::process::exit(1);
                }
            }
        }
        Some(("merge", sub_matches)) => {
            let dir = PathBuf::from(sub_matches.get_one::<String>("directory").unwrap());
            let output = PathBuf::from(sub_matches.get_one::<String>("output").unwrap());

            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }

            let chunks = get_chunks(&dir)?;
            if chunks.is_empty() {
                return Err(io::Error::new(io::ErrorKind::NotFound, "No chunks found matching the pattern"));
            }

            println!("{}", format!("🔗 Merging {} chunks into {}...\n", chunks.len(), output.display()).blue());

            let progress = ProgressBar::new(0).with_style(
                ProgressStyle::default_bar()
                    .template("📊 {bar:40} | {percent}% | {pos}/{len} chunks")
                    .unwrap()
                    .progress_chars("█░░"),
            );

            match merge(chunks.clone(), &output, progress).await {
                Ok(time) => {
                    println!("\n{}\n", "\n✅ Merge complete! 🎉".green().bold());
                    println!("  📦 Chunks merged: {}", chunks.len());
                    println!("  ⏱️  Time taken: {}s", time.round());
                    println!("  📁 Output file: {}\n", output.display());
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Error merging chunks:".red().bold(), e.to_string().red());
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
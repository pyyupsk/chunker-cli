use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::path::PathBuf;
use tokio;

mod chunker;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Command::new("chunker-cli")
        .about("🚀 A high-performance tool designed to split large files into smaller chunks and merge them back together.".cyan().to_string())
        .version("1.0.0")
        .subcommand(
            Command::new("split")
                .about("🔪 Split a file into smaller chunks")
                .arg(Arg::new("source").help("Source file path").required(true))
                .arg(Arg::new("output").short('o').long("output").help("Output directory for chunks"))
                .arg(Arg::new("concurrent").short('c').long("concurrent").help("Number of concurrent tasks").value_parser(clap::value_parser!(usize)))
                .arg(Arg::new("chunk_size").short('s').long("chunk-size").help("Custom chunk size in bytes (in binary), 25MB = 26214400").value_parser(clap::value_parser!(usize)))
        )
        .subcommand(
            Command::new("merge")
                .about("🧩 Merge file chunks back into a single file")
                .arg(Arg::new("directory").help("Directory containing chunk files").required(true))
                .arg(Arg::new("output").help("Output file path").required(true))
                .arg(Arg::new("buffer_size").short('b').long("buffer-size").help("Buffer size in bytes for reading/writing").value_parser(clap::value_parser!(usize)))
                .arg(Arg::new("cleanup").short('c').long("cleanup").help("Delete chunks after successful merge").action(clap::ArgAction::SetTrue))
        );

    let matches = app.get_matches();

    let progress_style = indicatif::ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}"
    )
    .unwrap()
    .progress_chars("#>-");

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
            let chunk_size = sub_matches
                .get_one::<usize>("chunk_size")
                .copied()
                .unwrap_or(25690112);

            std::fs::create_dir_all(&output_dir)?;

            println!(
                "{}",
                format!("📂 Preparing to split {}...\n", source.display()).green()
            );

            let progress = indicatif::ProgressBar::new(0).with_style(progress_style);

            match chunker::split(&source, &output_dir, concurrent, progress, chunk_size).await {
                Ok(result) => {
                    println!("\n{}\n", "\n✅ Split complete! 🎉".green().bold());
                    println!("  📦 Chunks created: {}", result.chunks);
                    println!("  ⏱️  Time taken: {}s", result.time.round());
                    println!("  📁 Output directory: {}\n", output_dir.display());
                }
                Err(e) => {
                    eprintln!(
                        "{} {}",
                        "❌ Error splitting file:".red().bold(),
                        e.to_string().red()
                    );
                    std::process::exit(1);
                }
            }
        }
        Some(("merge", sub_matches)) => {
            let dir = PathBuf::from(sub_matches.get_one::<String>("directory").unwrap());
            let output = PathBuf::from(sub_matches.get_one::<String>("output").unwrap());
            let buffer_size = sub_matches
                .get_one::<usize>("buffer_size")
                .copied()
                .unwrap_or(8388608);

            let cleanup = sub_matches.get_flag("cleanup");

            let chunks = chunker::get_chunks(&dir)?;
            if chunks.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No chunks found matching the pattern",
                ));
            }

            println!(
                "{}",
                format!(
                    "🔗 Merging {} chunks into {}...\n",
                    chunks.len(),
                    output.display()
                )
                .blue()
            );

            let progress = indicatif::ProgressBar::new(0).with_style(progress_style);

            match chunker::merge(chunks.clone(), &output, progress, buffer_size).await {
                Ok(time) => {
                    println!("\n{}\n", "\n✅ Merge complete! 🎉".green().bold());
                    println!("  📦 Chunks merged: {}", chunks.len());
                    println!("  ⏱️  Time taken: {}s", time.round());
                    println!("  📁 Output file: {}\n", output.display());

                    if cleanup {
                        for chunk in chunks {
                            if let Err(e) = fs::remove_file(&chunk) {
                                eprintln!(
                                    "Warning: Failed to remove chunk {}: {}",
                                    chunk.display(),
                                    e
                                );
                            }
                        }
                        if let Err(e) = fs::remove_dir(&dir) {
                            eprintln!("Warning: Failed to remove chunks directory: {}", e);
                        } else {
                            println!("🧹 Cleaned up chunks directory");
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} {}",
                        "❌ Error merging chunks:".red().bold(),
                        e.to_string().red()
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

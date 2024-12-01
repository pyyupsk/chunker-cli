use colored::*;
use indicatif::ProgressBar;
use std::path::PathBuf;

use crate::chunker;
use crate::utils;

pub async fn handle_split(sub_matches: &clap::ArgMatches) -> std::io::Result<()> {
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

    let progress = ProgressBar::new(0).with_style(utils::progress_style());

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

    Ok(())
}

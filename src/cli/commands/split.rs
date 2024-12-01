use colored::*;
use indicatif::ProgressBar;
use std::path::PathBuf;

use crate::split_file;
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
        .unwrap_or(4);
    let chunk_size = if let Some(size_str) = sub_matches.get_one::<String>("chunk_size") {
        utils::parse_size(size_str)?
    } else {
        24.0 * 1024.0 * 1024.0 // 24MB default
    };

    std::fs::create_dir_all(&output_dir)?;

    println!(
        "{}",
        format!("📂 Preparing to split {}...\n", source.display()).green()
    );

    let progress = ProgressBar::new(0).with_style(utils::progress_style());

    match split_file(&source, &output_dir, concurrent, chunk_size, progress).await {
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

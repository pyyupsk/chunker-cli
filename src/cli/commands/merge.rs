use colored::*;
use indicatif::ProgressBar;
use std::path::PathBuf;

use crate::merge_files;
use crate::utils;

pub async fn handle_merge(sub_matches: &clap::ArgMatches) -> std::io::Result<()> {
    let dir = PathBuf::from(sub_matches.get_one::<String>("directory").unwrap());
    let output = PathBuf::from(sub_matches.get_one::<String>("output").unwrap());
    let concurrent = sub_matches
        .get_one::<usize>("concurrent")
        .copied()
        .unwrap_or(4);
    let buffer_size = if let Some(size_str) = sub_matches.get_one::<String>("buffer_size") {
        utils::parse_size(size_str)?
    } else {
        8.0 * 1024.0 * 1024.0 // 8MB default
    };
    let cleanup = sub_matches.get_flag("cleanup");

    let chunks = utils::get_chunks(&dir)?;
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

    let progress = ProgressBar::new(0).with_style(utils::progress_style());

    match merge_files(chunks.clone(), &output, concurrent, buffer_size, progress).await {
        Ok(time) => {
            println!("\n{}\n", "\n✅ Merge complete! 🎉".green().bold());
            println!("  📦 Chunks merged: {}", chunks.len());
            println!("  ⏱️  Time taken: {}s", time.round());
            println!("  📁 Output file: {}\n", output.display());

            if cleanup {
                utils::cleanup_chunks(&chunks, &dir);
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

    Ok(())
}

use clap::{Arg, Command};
use colored::*;
use tokio;

mod chunker;
mod functions;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Command::new("chunker-cli")
        .about(
            "🚀 A blazing-fast tool to split large files into smaller chunks and merge them back seamlessly."
                .cyan()
                .to_string(),
        )
        .version("1.0.0")
        .subcommand(
            Command::new("split")
                .about("🔪 Split a file into smaller chunks for easier management or transfer")
                .arg(Arg::new("source").help("Path to the source file to split").required(true))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Specify the directory to save the chunks"),
                )
                .arg(
                    Arg::new("concurrent")
                        .short('c')
                        .long("concurrent")
                        .help("Set the number of concurrent tasks for splitting")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("chunk_size")
                        .short('s')
                        .long("chunk-size")
                        .help("Size of each chunk in bytes (e.g., 25MB = 26214400)")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("merge")
                .about("🧩 Combine smaller chunks back into a single file")
                .arg(
                    Arg::new("directory")
                        .help("Path to the directory containing chunk files")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .help("Path for the resulting merged file")
                        .required(true),
                )
                .arg(
                    Arg::new("buffer_size")
                        .short('b')
                        .long("buffer-size")
                        .help("Buffer size in bytes for reading and writing data")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("cleanup")
                        .short('c')
                        .long("cleanup")
                        .help("Automatically delete chunks after a successful merge")
                        .action(clap::ArgAction::SetTrue),
                ),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => functions::handle_split(sub_matches).await,
        Some(("merge", sub_matches)) => functions::handle_merge(sub_matches).await,
        _ => unreachable!(),
    }
}

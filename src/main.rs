use clap::{Arg, Command};
use colored::*;
use tokio;

mod chunker;
mod functions;
mod utils;

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

    match matches.subcommand() {
        Some(("split", sub_matches)) => functions::handle_split(sub_matches).await,
        Some(("merge", sub_matches)) => functions::handle_merge(sub_matches).await,
        _ => unreachable!(),
    }
}

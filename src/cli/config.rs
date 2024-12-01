use clap::{Arg, Command};
use colored::*;

pub fn build_cli() -> Command {
    Command::new("chunker-cli")
        .about(
            "🚀 A blazing-fast tool to split large files into smaller chunks and merge them back seamlessly."
                .cyan()
                .to_string(),
        )
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown"))
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
                        .help("Size of each chunk (e.g., 10MB, 1GB)")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new()),
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
                    Arg::new("concurrent")
                        .short('c')
                        .long("concurrent")
                        .help("Set the number of concurrent tasks for merging")
                        .value_parser(clap::value_parser!(usize)),
                )
                .arg(
                    Arg::new("buffer_size")
                        .short('b')
                        .long("buffer-size")
                        .help("Buffer size for reading and writing data (e.g., 8MB, 1GB)")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("cleanup")
                        .long("cleanup")
                        .short('C')
                        .help("Automatically delete chunks after a successful merge")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
}

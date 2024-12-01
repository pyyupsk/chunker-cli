use chunker_cli::cli::{build_cli, handle_merge, handle_split};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => handle_split(sub_matches).await,
        Some(("merge", sub_matches)) => handle_merge(sub_matches).await,
        _ => unreachable!(),
    }
}

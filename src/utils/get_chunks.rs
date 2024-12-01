use std::path::{Path, PathBuf};

pub fn get_chunks(output_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut chunks: Vec<_> = std::fs::read_dir(output_dir)?
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

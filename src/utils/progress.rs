use indicatif::ProgressStyle;

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}",
    )
    .unwrap()
    .progress_chars("#>-")
}

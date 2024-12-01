use std::io;

pub fn parse_size(size: &str) -> io::Result<f64> {
    let size = size.trim().to_uppercase();
    let chars = size.chars().collect::<Vec<_>>();

    let split_idx = chars
        .iter()
        .position(|c| !c.is_digit(10) && *c != '.')
        .unwrap_or(chars.len());

    let number: f64 = chars[..split_idx]
        .iter()
        .collect::<String>()
        .parse()
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid number in size specification",
            )
        })?;

    let multiplier = if split_idx < chars.len() {
        match chars[split_idx..].iter().collect::<String>().as_str() {
            "KB" => 1024.0,
            "MB" => 1024.0 * 1024.0,
            "GB" => 1024.0 * 1024.0 * 1024.0,
            "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
            "" => 1.0,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid size unit. Use KB, MB, GB, or TB",
                ))
            }
        }
    } else {
        1.0
    };

    Ok(number * multiplier)
}

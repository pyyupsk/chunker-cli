use std::io;

pub fn parse_size(size: &str) -> io::Result<usize> {
    let size = size.trim().to_uppercase();
    let mut chars = size.chars().collect::<Vec<_>>();

    let split_idx = chars
        .iter()
        .position(|c| !c.is_digit(10))
        .unwrap_or(chars.len());

    let number: usize = chars[..split_idx]
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
        if chars.last() == Some(&'B') {
            chars.pop();
        }

        match chars[split_idx..].iter().collect::<String>().as_str() {
            "K" => 1024,
            "M" => 1024 * 1024,
            "G" => 1024 * 1024 * 1024,
            "T" => 1024 * 1024 * 1024 * 1024,
            "" => 1,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid size unit. Use K, M, G, or T",
                ))
            }
        }
    } else {
        1
    };

    Ok(number * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("1024").unwrap(), 1024);
        assert_eq!(parse_size("1K").unwrap(), 1024);
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("1M").unwrap(), 1024 * 1024);
        assert_eq!(parse_size("1MB").unwrap(), 1024 * 1024);
        assert_eq!(parse_size("1G").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("2MB").unwrap(), 2 * 1024 * 1024);
        assert!(parse_size("invalid").is_err());
        assert!(parse_size("1XB").is_err());
    }
}

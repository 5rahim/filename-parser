use nom::character::complete::{digit1, multispace0};
use nom::combinator::recognize;
use nom::sequence::tuple;
use regex::Regex;

pub fn is_digits(input: &str) -> bool {
    match recognize(tuple((digit1::<_, ()>, multispace0)))(input) {
        Ok((remaining, _matched)) => remaining.is_empty(),
        Err(_) => false,
    }
}

pub fn extract_season_and_episode(input: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r"^[Ss]?(\d+)([Eex])(\d+\D?\d{1,4})?$").unwrap();
    if let Some(captures) = re.captures(input) {
        let season = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let separator = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let episode = captures.get(3).map_or("", |m| m.as_str()).to_string();
        Some((season, separator, episode))
    } else {
        None
    }
}

pub fn is_number_like(input: &str) -> bool {
    let mut chars = input.chars();

    // Check if the first character is a digit
    if let Some(first_char) = chars.next() {
        if !first_char.is_digit(10) {
            return false;
        }
    } else {
        // Empty string, not meeting the criteria
        return false;
    }

    // Check for ordinal
    if is_ordinal_number(input) {
        return true;
    }

    // Check if the last character is a digit or "'"
    if let Some(last_char) = chars.next_back() {
        if last_char == '\'' {
            return true;
        }
        if !last_char.is_digit(10) {
            return false;
        }
    } else {
        // Single character string, meeting the criteria
        return true;
    }

    // Count non-digit characters in the remaining string
    let non_digits = chars.filter(|c| !c.is_digit(10)).count();

    non_digits == 1
}

pub fn is_ordinal_number(input: &str) -> bool {
    return input.to_lowercase().ends_with("th") ||
        input.to_lowercase().ends_with("st") ||
        input.to_lowercase().ends_with("rd")
}

pub fn is_number_or_like(input: &str) -> bool {
    return is_number_like(input) || is_digits(input)
}

pub fn number_is_zero_padded(input: &str) -> bool {
    if let Ok(_) = input.parse::<u32>() {
        return input.starts_with('0')
    }
    false
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
enum VideoResolution {
    Standard(u32, u32),
    HD(u32),
    _4K,
}

fn parse_resolution(input: &str) -> Option<VideoResolution> {
    let standard_regex = Regex::new(r"^\s*(\d+)\s*x\s*(\d+)\s*$").unwrap();
    let hd_regex = Regex::new(r"^\s*(\d+)\s*[pP]\s*$").unwrap();
    let _4k_regex = Regex::new(r"^\s*4K\s*$").unwrap();

    if let Some(captures) = standard_regex.captures(input) {
        let width = captures[1].parse().ok()?;
        let height = captures[2].parse().ok()?;
        Some(VideoResolution::Standard(width, height))
    } else if let Some(captures) = hd_regex.captures(input) {
        let height = captures[1].parse().ok()?;
        Some(VideoResolution::HD(height))
    } else if _4k_regex.is_match(input) {
        Some(VideoResolution::_4K)
    } else {
        None
    }
}

pub fn is_crc32(s: &str) -> bool {
    s.len() == 8 && is_hexadecimal_string(s)
}

pub fn is_hexadecimal_string(s: &str) -> bool {
    u32::from_str_radix(s, 16).is_ok()
}

pub fn is_video_resolution(input: &str) -> bool {
    parse_resolution(input).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_episodes() {
        assert_eq!(extract_season_and_episode("S01E01"), Some(("01".to_string(), "E".to_string(), "01".to_string())));
        assert_eq!(extract_season_and_episode("S01E01'"), Some(("01".to_string(), "E".to_string(), "01".to_string())));
        assert_eq!(extract_season_and_episode("01E01"), Some(("01".to_string(), "E".to_string(), "01".to_string())));
        assert_eq!(extract_season_and_episode("S01x01"), Some(("01".to_string(), "x".to_string(), "01".to_string())));
        assert_eq!(extract_season_and_episode("01x01"), Some(("01".to_string(), "x".to_string(), "01".to_string())));
        assert_eq!(extract_season_and_episode("03E03v3"), Some(("03".to_string(), "E".to_string(), "03".to_string())));
        assert_eq!(extract_season_and_episode("10E05x2"), Some(("10".to_string(), "E".to_string(), "05".to_string())));
    }

    #[test]
    fn test_invalid_episodes() {
        assert_eq!(extract_season_and_episode("05E02a"), None);
        assert_eq!(extract_season_and_episode("ABCDEF"), None);
    }
}
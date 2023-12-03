use regex::Regex;

pub fn replace_case_insensitive(input: &str, pattern: &str, replacement: &str) -> String {
    let regex_pattern = Regex::new(&format!(r"(?i){}", regex::escape(pattern))).unwrap();
    regex_pattern.replace_all(input, replacement).to_string()
}
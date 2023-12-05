#![allow(dead_code)]

use crate::keyword::{KeywordCategory, KeywordKind, KeywordPriority};
use crate::metadata::MetadataKind;
use crate::token::{Token, TokenCategory, TokenKind};
use crate::token_helper::{is_crc32, is_video_resolution, number_is_zero_padded};
use crate::token_manager::TokenManager;
use crate::tokenizer;

#[derive(Debug)]
pub struct Parser {
    pub token_manager: TokenManager,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            token_manager: TokenManager::new(tokens)
        }
    }

    pub fn parse(&mut self) {
        self.parse_file_checksum_and_video_resolution();

        self.parse_keywords(KeywordPriority::Normal);

        self.normalize_keywords();

        self.parse_season();

        // other things

        // self.parse_keywords(KeywordPriority::Low);
    }

    fn parse_season(&mut self) {
        if self.token_manager.has_token_with_metadata_kind(MetadataKind::Season) {
            return ();
        }

        let tokens = self.token_manager.get_tokens();
        let mut iterator = tokens.iter();
        // Get SeasonPrefix
        if let Some(season_prefix_token) = iterator.find(|t| t.category.is_keyword(KeywordCategory::SeasonPrefix)) {
            // Get Keyword from it
            if let Some(keyword) = season_prefix_token.category.get_keyword() {
                match keyword.kind {
                    KeywordKind::Standalone => {} // invalid
                    KeywordKind::Combined { .. } => { // e.g. S01
                        // check
                    }
                    KeywordKind::OrdinalSuffix => {

                    }
                    _ => { // e.g. Season 1 or Seasons 1-4
                        // Check range
                        loop {
                            if let Some(number_range_tokens) = self.get_number_range_after(season_prefix_token) {
                                // If the keyword ends with "s" (e.g. Seasons), then we are sure that it is a range
                                if keyword.value.ends_with("S") {
                                    self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                    self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::Season));
                                    break;
                                }
                                let first_number_is_zero_padded = number_is_zero_padded(number_range_tokens[0].value.as_str());
                                let second_number_is_zero_padded = number_is_zero_padded(number_range_tokens[1].value.as_str());

                                // Check that the right side of the range isn't an episode number
                                // e.g. If we encounter this "1 - 03" then 05 might be the episode number,
                                if !first_number_is_zero_padded && second_number_is_zero_padded {
                                    self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                    self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::EpisodeNumber));
                                    break;
                                } else if first_number_is_zero_padded && second_number_is_zero_padded {

                                    // "01 - 03" (with delimiter) -> Season & Episode
                                    if let Some(_) = self.get_delimiter_before(&number_range_tokens[1]) {
                                        self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                        self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::EpisodeNumber));
                                        break;
                                    }
                                    // "01-03" (without delimiter) -> Season & Season
                                    self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                    self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::Season));
                                    break;
                                } else {
                                    // if we encounter "[1,01] - 12" (>= 10) (with or without delimiters) -> Season & Episode
                                    if let Ok(second_number) = number_range_tokens[1].value.parse::<u32>() {
                                        if second_number > 10 {
                                            self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                            self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::EpisodeNumber));
                                        }
                                    }
                                    // if we encounter "1 - 5" (<10) (with or without delimiters) -> Season & Season
                                    self.token_manager.update_token_category(number_range_tokens[0].uuid, TokenCategory::Known(MetadataKind::Season));
                                    self.token_manager.update_token_category(number_range_tokens[1].uuid, TokenCategory::Known(MetadataKind::Season));
                                    break;
                                }
                            };
                            if let Some(number_token) = self.get_number_or_like_after(season_prefix_token) {
                                self.token_manager.update_token_category(number_token.uuid, TokenCategory::Known(MetadataKind::Season));
                                break;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    ///
    /// Identify keywords for each token.
    /// When a keyword is found, update that token's category
    ///
    fn parse_keywords(&mut self, keyword_priority: KeywordPriority) {
        let tokens = self.token_manager.get_tokens();
        for token in tokens.iter() {
            // Only parse unknown tokens
            if !token.category.is_known() {
                if let Some(ret_tokens) = self.token_manager.identify_keyword(token) {
                    match ret_tokens.len() {
                        1 => {
                            let ret_token = ret_tokens[0].clone();
                            match ret_token.category {
                                TokenCategory::Keyword(keyword) => {
                                    if keyword.priority == keyword_priority {
                                        self.token_manager.update_token_category(token.uuid, TokenCategory::Keyword(keyword))
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            self.token_manager.update_token_category(token.uuid, TokenCategory::TokenParts(ret_tokens.clone()))
                        }
                        // _ => {}
                    }
                }
            }
        }
    }

    ///
    /// Flatten tokens whose category is TokenCategory::TokenParts.
    ///
    fn normalize_keywords(&mut self) {
        let tokens = self.token_manager.get_tokens();
        let iterator = tokens.iter();
        for token in iterator.filter(|t| t.category.is_token_parts()) {
            // we don't skip delimiters to get real index
            if let Some(token_index) = self.token_manager.get_index_of_token(token, false) {
                if let Some(token_parts) = token.category.get_token_parts() {
                    self.token_manager.flatten_token_at(token_index, token_parts)
                }
            }
        }
    }

    ///
    /// Parse FileChecksum and VideoResolution tokens
    ///
    fn parse_file_checksum_and_video_resolution(&mut self) {
        let tokens = self.token_manager.get_tokens();
        for token in tokens.iter() {
            if !token.category.is_known() && is_crc32(token.value.as_str()) {
                self.token_manager.update_token_category(token.uuid, TokenCategory::Known(MetadataKind::FileChecksum))
            }
            if is_video_resolution(token.value.as_str()) {
                self.token_manager.update_token_category(token.uuid, TokenCategory::Known(MetadataKind::VideoResolution))
            }
        }
    }

    /// Get a potential number with decimal, do not ignore delimiters.
    /// Tokens should be of TokenKind::Number.
    /// e.g. "1" "." "5"
    fn get_number_with_decimal_after(&mut self, token: &Token) -> Option<Vec<Token>> {
        if let Some(index) = self.token_manager.get_index_of_token(token, false) {
            if let Some(tokens) = self.token_manager.get_matching_tokens_after(index, vec![TokenCategory::Unknown, TokenCategory::Delimiter, TokenCategory::Unknown], false) {
                if tokens.len() != 3 {
                    return None;
                }
                if let Some(delimiter) = tokens.iter().find(|t| t.category.is_delimiter()) {
                    if delimiter.value == ".".to_string() {
                        return Some(tokens);
                    }
                }
            }
        }
        return None;
    }

    /// Get a potential number range, ignore delimiters.
    /// Tokens should be of TokenKind::Number.
    /// e.g. "1" (delimiter)? "-" (delimiter)? "3"
    fn get_number_range_after(&mut self, token: &Token) -> Option<Vec<Token>> {
        if let Some(index) = self.token_manager.get_index_of_token(token, true) {
            if let Some(tokens) = self.token_manager.get_matching_tokens_after(index, vec![TokenCategory::Unknown, TokenCategory::Separator, TokenCategory::Unknown], true) {
                if tokens.len() != 3 {
                    return None;
                }
                let number_tokens: Vec<Token> = tokens.iter().filter(|t| t.kind.is_number()).cloned().collect();
                if number_tokens.len() != 2 {
                    return None;
                }
                return Some(number_tokens);
            }
        }
        return None;
    }

    /// Get a potential number
    fn get_number_or_like_after(&mut self, token: &Token) -> Option<Token> {
        if let Some(index) = self.token_manager.get_index_of_token(token, true) {
            if let Some(token) = self.token_manager.get_token_after(index, true) {
                if token.kind.is_number_or_like() {
                    return Some(token);
                }
            }
        }
        return None;
    }

    /// Get a potential number
    fn get_delimiter_before(&mut self, token: &Token) -> Option<Token> {
        if let Some(index) = self.token_manager.get_index_of_token(token, false) {
            if let Some(token) = self.token_manager.get_token_before(index, false) {
                if token.category.is_delimiter() {
                    return Some(token);
                }
            }
        }
        return None;
    }
}

#[test]
fn test_parsing() {
    let input = String::from("[HorribleSubs] Tower of Druaga - Sword of Uruk - S01E04 [480p]");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    assert!(true);
    println!("{:#?}", parser.token_manager.get_tokens());
}

#[test]
fn test_parsing_00() {
    let input = String::from("[SubsPlease] Jujutsu Kaisen Season 2 - 01 [1080p]");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    assert!(true);
    println!("{:#?}", parser.token_manager.get_tokens());
}

#[test]
fn test_parsing_01() {
    let input = String::from("[ST]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    assert!(true);
    println!("{:#?}", parser.token_manager.get_tokens());
}

#[test]
fn test_parsing_02() {
    let input = String::from("Violet.Evergarden.The.Movie.1080p.Dual.Audio.BDRip.10.bits.DD.x265-EMBER");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    assert!(true);
    println!("{:#?}", parser.token_manager.get_tokens());
}

#[test]
fn test_parsing_03() {
    let input = String::from("[SubsPlease] Jujutsu Kaisen Seasons 01 - 03 [1080p]");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    assert!(true);
    println!("{:#?}", parser.token_manager.get_tokens());
}
#![allow(dead_code)]

use crate::keyword::{KeywordCategory, KeywordPriority};
use crate::metadata::MetadataKind;
use crate::token::{Token, TokenCategory};
use crate::token_helper::{is_crc32, is_video_resolution};
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
        if let Some(season_prefix_token) = iterator.find(|t| t.category.is_keyword(KeywordCategory::SeasonPrefix)) {
            println!("Found season prefix {:?}", season_prefix_token)
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
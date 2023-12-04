#![allow(dead_code)]

use crate::keyword::KeywordPriority;
use crate::metadata::MetadataKind;
use crate::token::{Token, TokenCategory};
use crate::token_helper::is_crc32;
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
        self.parse_file_checksum();

        self.parse_keywords(KeywordPriority::Normal);

        // other things

        // self.parse_keywords(KeywordPriority::Low);
    }

    fn parse_keywords(&mut self, _keyword_priority: KeywordPriority) {
        let len = self.token_manager.tokens.len();

        // for i in 0..len {
        //     println!("{:?}", self.token_manager.identify_keyword(&self.token_manager.tokens[i].to_owned()))
        // }

        // println!("--------------------------------------")
    }

    fn parse_file_checksum(&mut self) {
        let tokens = self.token_manager.get_tokens();
        for token in tokens.iter() {
            if !token.is_known() && is_crc32(token.value.as_str()) {
                self.token_manager.update_token_category(token.uuid, TokenCategory::Known(MetadataKind::FileChecksum))
            }
        }
    }
}

#[test]
fn test_parsing() {
    let input = String::from("[HorribleSubs] Tower of Druaga - Sword of Uruk - S01 04 [480p]");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("{:#?}", parser.token_manager.get_tokens());
    assert!(true);
}

#[test]
fn test_parsing_01() {
    let input = String::from("[ST]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv");
    let tokens = tokenizer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("{:#?}", parser.token_manager.get_tokens());
    assert!(true);
}
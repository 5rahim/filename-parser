#![allow(dead_code)]

use crate::keyword::{KeywordCategory, KeywordPriority};
use crate::token::{Token, TokenCategory};
use crate::token_manager::TokenManager;
use crate::tokenizer;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Parser {
    pub token_manager: TokenManager
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            token_manager: TokenManager::new(tokens)
        }
    }

    pub fn parse(&mut self) {

        self.parse_keywords(KeywordPriority::Normal);

        // other things

        self.parse_keywords(KeywordPriority::Low);

    }

    fn parse_keywords(&mut self, keyword_priority: KeywordPriority) {
        let iterator = self.token_manager.tokens.iter_mut();
        for token in iterator {
            match token.category {
                TokenCategory::Unknown => {

                    if let Some(keyword) = self.token_manager.keyword_manager.find(token.value.borrow()) {

                        if keyword.priority != keyword_priority {
                            continue
                        }

                        match keyword.category {
                            KeywordCategory::SeasonPrefix => {
                                match keyword.kind {
                                    _ => {}
                                }
                                token.category = TokenCategory::Keyword(keyword.clone())
                            }
                            KeywordCategory::EpisodePrefix => {}
                            KeywordCategory::VolumePrefix => {}
                            KeywordCategory::PartPrefix => {}
                            KeywordCategory::AnimeType => {}
                            KeywordCategory::VideoTerm => {}
                            KeywordCategory::AudioTerm => {}
                            KeywordCategory::DeviceCompat => {}
                            KeywordCategory::FileExtension => {}
                            KeywordCategory::Language => {}
                            KeywordCategory::ReleaseInfo => {}
                            // KeywordCategory::ReleaseVersion => {}
                            KeywordCategory::ReleaseGroup => {
                                token.category = TokenCategory::Keyword(keyword.clone())
                                // We still need to check for non-keyword release groups
                            }
                            KeywordCategory::Subtitles => {}
                            KeywordCategory::Source => {}
                            _ => {}
                        }
                    }

                }
                _ => {}
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
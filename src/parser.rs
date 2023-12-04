#![allow(dead_code)]

use crate::keyword::KeywordPriority;
use crate::token::Token;
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
        self.parse_keywords(KeywordPriority::Normal);

        // other things

        self.parse_keywords(KeywordPriority::Low);
    }

    fn parse_keywords(&mut self, _keyword_priority: KeywordPriority) {}
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
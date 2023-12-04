#![allow(dead_code)]

use crate::keyword::{Keyword, KeywordCategory, KeywordKind, KeywordPriority};
use crate::keyword_manager::KeywordManager;
use crate::metadata::MetadataKind;
use crate::token::{Token, TokenCategory, TokenKind};
use crate::token_helper::{extract_season_and_episode, is_digits, is_number_like, is_number_or_like, is_ordinal_number, is_video_resolution};
use crate::tokenizer;
use crate::utils::replace_case_insensitive;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenManager {
    pub tokens: Vec<Token>,
    pub keyword_manager: KeywordManager,
}

///
/// TokenManager is tasked
///
impl TokenManager {
    pub fn new(tokens: Vec<Token>) -> TokenManager {
        let mut tm = TokenManager {
            tokens,
            keyword_manager: KeywordManager::new(),
        };

        tm.pre_processing();

        tm
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        return self.tokens.clone();
    }

    pub fn update_token_category(&mut self, id: Uuid, token_category: TokenCategory) {
        if let Some(token) = self.tokens.iter_mut().find(|t| t.uuid == id) {
            token.category = token_category
        }
    }

    ///
    /// Identifies whether the token is a keyword by comparing it to all keywords.
    /// It will prefer "combination" keywords over "standalone" ones.
    ///
    pub fn identify_keyword(&mut self, token: &Token) -> Option<Vec<Token>> {

        if token.is_known() {
            return None;
        }

        // Check if the token is a video resolution
        if is_video_resolution(token.value.as_str()) {
            return Some(vec![
                Token {
                    category: TokenCategory::Keyword(
                        Keyword::new(token.value.to_string(), KeywordCategory::VideoTerm, KeywordKind::Standalone, KeywordPriority::Normal)
                    ),
                    ..token.clone()
                }
            ]);
        }

        // Find possible keywords by value
        let keyword_ret = self.keyword_manager.find_many(token.value.as_str());
        let mut found: Option<Vec<Token>> = None;
        // If we were able to find some keywords, we will validate them based on their KeywordKind
        if let Some(keywords) = keyword_ret {
            let iter = keywords.iter();
            for keyword in iter {
                // Validate the keyword
                if let Some(ret) = self.validate_keyword(token, keyword) {
                    match ret[0].clone().category {
                        TokenCategory::Keyword(kw) => {
                            match kw.kind {
                                // If the validated keyword is "standalone", make sure we didn't
                                // already validate another kind.
                                KeywordKind::Standalone => {
                                    if found == None {
                                        found = Some(ret)
                                    }
                                }
                                _ => {
                                    found = Some(ret)
                                }
                            }
                        }
                        _ => {}
                    };
                }
            }
        }
        return found;
    }
    // pub fn identify_keyword(&mut self, token: &Token) -> Option<Vec<Token>> {
    //     // Find the keyword by value
    //     let keyword_ret = self.keyword_manager.find(token.value.as_str());
    //     // If the keyword exist, we check its category
    //     if let Some(keyword) = keyword_ret {
    //         return self.validate_keyword(token, &keyword);
    //     }
    //     return None;
    // }

    /// De-duplicating keyword matches
    /// Flattening tokens whose TokenCategory is TokenCategory::TokenParts
    pub fn normalize(&mut self, _token: &Token, _validated_tokens: Vec<Token>) {}


    ///
    /// If the returned vector has a single Token, the input token should be of TokenCategory::Keyword(keyword).
    ///
    /// If the returned vector has more than one Token, the input token should be of TokenCategory::TokenParts(Vec<Token>).
    ///
    fn validate_keyword(&mut self, token: &Token, keyword: &Keyword) -> Option<Vec<Token>> {
        return match keyword.kind {

            // If the keyword is of combined type and the next token should be NumberLike
            // e.g: "S" or "E"
            KeywordKind::Combined { next_token_kind: TokenKind::NumberLike } => {

                // Directly try parsing episode, season
                if let Some(res) = extract_season_and_episode(token.value.as_str()) {
                    // Here is the only place where we will immediately employ TokenCategory::Known
                    // since we are sure
                    return Some(vec![
                        Token::new_with_kind(keyword.value.to_string(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                        Token::new_with_kind(res.0.as_str(), TokenCategory::Known(MetadataKind::Season), TokenKind::Number),
                        Token::new_with_kind(
                            res.1.as_str(),
                            TokenCategory::Keyword(
                                Keyword::new(res.1.to_string(), KeywordCategory::EpisodePrefix, KeywordKind::Combined { next_token_kind: TokenKind::NumberLike }, KeywordPriority::Normal)
                            ),
                            TokenKind::SingleCharacter),
                        Token::new_with_kind(res.2.as_str(), TokenCategory::Known(MetadataKind::EpisodeNumber), if is_digits(res.2.as_str()) { TokenKind::Number } else { TokenKind::NumberLike }),
                    ]);
                }

                // Split and check adjacent token e.g. S01 -> S 01
                let suffix = replace_case_insensitive(token.value.as_str(), keyword.value.as_str(), "");
                if !suffix.is_empty() && is_number_or_like(suffix.as_str()) {
                    return Some(vec![
                        Token::new_with_kind(keyword.value.to_string(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                        Token::new_with_kind(suffix.as_str(), TokenCategory::Unknown, if is_digits(suffix.as_str()) { TokenKind::Number } else { TokenKind::NumberLike }),
                    ]);
                }


                return None;
            }

            // If the keyword is of combined or separated type and the next token should be NumberLike
            KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike } => {

                // First we check for combined e.g. OP1
                let suffix = replace_case_insensitive(token.value.as_str(), keyword.value.as_str(), "");
                if !suffix.is_empty() && is_number_or_like(suffix.as_str()) {
                    return Some(
                        vec![
                            Token::new_with_kind(keyword.value.clone(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                            Token::new_with_kind(suffix.as_str(), TokenCategory::Unknown, if is_digits(suffix.as_str()) { TokenKind::Number } else { TokenKind::NumberLike }),
                        ]
                    );
                }

                // Then we check for separated

                if self.next_token_is_number_like(token) {
                    return Some(vec![
                        Token::new_with_kind(keyword.value.clone(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                    ]);
                }

                return None;
            }
            KeywordKind::Separated { next_token_kind: TokenKind::NumberLike } => {
                if self.next_token_is_number_like(token) {
                    return Some(vec![
                        Token::new_with_kind(keyword.value.clone(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                    ]);
                }

                return None;
            }
            KeywordKind::OrdinalSuffix => {
                println!("{:?}", keyword);

                if self.previous_token_is_ordinal_number(token) {
                    return Some(vec![
                        Token::new_with_kind(keyword.value.clone(), TokenCategory::Keyword(keyword.clone()), TokenKind::String),
                    ]);
                }

                return None;
            }
            // If the keyword is standalone, we don't need to check adjacent tokens,
            KeywordKind::Standalone => Some(vec![
                Token::new_with_kind(token.value.clone(), TokenCategory::Keyword(keyword.clone()), TokenKind::String)
            ]),
            _ => None
        };
    }

    //--------------------

    pub fn get_token_by_metadata_kind(&mut self, kind: MetadataKind) -> Option<Token> {
        let mut m = self.tokens.iter();
        m.find(|t| t.has_metadata_kind(kind)).cloned()
    }

    pub fn has_token_with_metadata_kind(&mut self, kind: MetadataKind) -> bool {
        self.get_token_by_metadata_kind(kind).is_some()
    }

    pub fn get_index_of_token(&mut self, token: &Token, skip_delimiter: bool) -> Option<usize> {
        let mut m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        m.position(|t| t.uuid == token.uuid)
    }

    pub fn get_known_token_after(&mut self, index: usize, category: TokenCategory, skip_delimiter: bool) -> Option<Token> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let mut iter = m.skip(index + 1);

        iter.find(|t| t.category == category).cloned()
    }

    pub fn get_known_token_before(&mut self, index: usize, category: TokenCategory, skip_delimiter: bool) -> Option<Token> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let count = m.clone().count();
        let mut iter = m.rev().skip(count - index);
        iter.find(|t| t.category == category).cloned()
    }

    pub fn get_token_after(&mut self, index: usize, skip_delimiter: bool) -> Option<Token> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let mut iter = m.skip(index + 1);
        iter.next().cloned()
    }

    pub fn get_token_before(&mut self, index: usize, skip_delimiter: bool) -> Option<Token> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let count = m.clone().count();
        let mut iter = m.rev().skip(count - index);
        iter.next().cloned()
    }

    pub fn get_tokens_until_category(&mut self, index: usize, category: TokenCategory, skip_delimiter: bool) -> Option<Vec<Token>> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let iter = m.skip(index + 1);
        let tokens = iter
            .take_while(|t| t.category != category)
            .cloned()
            .collect::<Vec<Token>>();

        if tokens.is_empty() {
            None
        } else {
            Some(tokens)
        }
    }

    pub fn get_next_token_from(&mut self, index: usize, skip_delimiter: bool) -> Option<Token> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let mut iter = m.skip(index + 1);
        iter.next().cloned()
    }

    // TODO redo
    pub fn get_matching_tokens_after(&mut self, index: usize, sequence: Vec<TokenCategory>, skip_delimiter: bool) -> Option<Vec<Token>> {
        let m = self.tokens.iter().filter(|t| if skip_delimiter { t.category != TokenCategory::Delimiter } else { true });
        let iter = m.skip(index + 1);

        let matching_tokens = iter
            .take_while(|t| sequence.contains(&t.category))
            .cloned()
            .collect::<Vec<Token>>();

        if matching_tokens.is_empty() || matching_tokens.len() != sequence.len() {
            return None;
        }

        Some(matching_tokens)
    }

    //------------ Private -------------

    ///
    /// Checks if the following token (no delimiter) is of TokenKind::Number or TokenKind::NumberLike
    ///
    fn next_token_is_number_like(&mut self, token: &Token) -> bool {
        // Check if the following token is NumberLike
        return if let Some(token_idx) = self.get_index_of_token(&token, true) {
            if let Some(next_token) = self.get_token_after(token_idx, true) {
                match next_token.kind {
                    TokenKind::Number => true,
                    TokenKind::NumberLike => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        };
    }

    ///
    /// Checks if the preceding token (no delimiter) is of TokenKind::Number or TokenKind::NumberLike
    ///
    fn previous_token_is_ordinal_number(&mut self, token: &Token) -> bool {
        // Check if the following token is NumberLike
        return if let Some(token_idx) = self.get_index_of_token(&token, true) {
            if let Some(prev_token) = self.get_token_before(token_idx, true) {
                match prev_token.kind {
                    TokenKind::NumberLike => {
                        if is_ordinal_number(prev_token.value.as_str()) {
                            return true;
                        }
                        false
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        };
    }

    /// Identify the kinds of each token.
    /// This is done when the TokenManager is instantiated.
    fn pre_processing(&mut self) {
        let len = self.tokens.len();
        for i in 0..len {
            if is_digits(self.tokens[i].value.as_str()) {
                self.tokens[i].kind = TokenKind::Number;
                //
            } else if is_number_like(self.tokens[i].value.as_str()) {
                self.tokens[i].kind = TokenKind::NumberLike
                //
            } else if self.tokens[i].value.len() == 1 {
                self.tokens[i].kind = TokenKind::SingleCharacter
                //
            } else {
                self.tokens[i].kind = TokenKind::String
                //
            }
        }
    }
}

#[test]
fn test_new_token_manager() {
    let input = String::from("[HorribleSubs]_Tower_of_Druaga_-_Sword_of_Uruk_-_04+Movie_[480p]");

    let tokens = tokenizer::tokenize(&input);

    let _token_manager = TokenManager::new(tokens.clone());

    assert!(true);
}

#[test]
fn test_token_kind_number_like() {
    let input = String::from("01v2 05' 01x05"); // 1.5 won't work because . is a separator

    let tokens = tokenizer::tokenize(&input);

    let mut token_manager = TokenManager::new(tokens.clone());

    println!("{:#?}", token_manager.get_tokens());

    for token in token_manager.get_tokens() {
        if token.value != " " {
            assert_eq!(TokenKind::NumberLike, token.kind)
        }
    }
}

#[test]
fn test_token_utils() {
    // delimiter skipped
    test_get_token_after("Title - 4th Season", 1, "4th");
    test_get_token_before("Title - 4th Season", 3, "4th");
    test_get_token_after("Season 1", 0, "1");
    test_get_token_after("One Two Three Four", 1, "Three");
}

#[test]
fn test_identify_keyword() {
    test_identify_keyword_combined("OP1 OVA01 Season1 E1 Ep1", TokenKind::Number);
    test_identify_keyword_combined("OP1v2", TokenKind::NumberLike);
    test_identify_keyword_combined("E01v2", TokenKind::NumberLike);
    test_identify_keyword_combined_or_separated("Season1");
    test_identify_keyword_combined_or_separated("Season 1");
    test_identify_keyword_combined_or_separated("Movie 01");
    test_identify_keyword_combined_or_separated("OVA 01");
    test_identify_keyword_deeply_combined("S01E01", vec![TokenKind::Number, TokenKind::Number]);
    test_identify_keyword_deeply_combined("S01E01v2", vec![TokenKind::Number, TokenKind::NumberLike]);
    test_identify_keyword_deeply_combined("S01x01v2", vec![TokenKind::Number, TokenKind::NumberLike]);
}

#[test]
fn test_identify_keyword_01() {
    test_identify_keyword_ordinal_suffix("4th season");
}

#[test]
#[should_panic]
fn test_identify_keyword_panic_00() {
    test_identify_keyword_combined("Sword", TokenKind::String); // should not detect "S" as season prefix
}

#[test]
#[should_panic]
fn test_identify_keyword_panic_01() {
    test_identify_keyword_combined("OPv2", TokenKind::Number);
}

#[test]
#[should_panic]
fn test_identify_keyword_panic_02() {
    test_identify_keyword_combined_or_separated("Season season");
}

fn test_identify_keyword_deeply_combined(input: &str, number_kinds: Vec<TokenKind>) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    let split_tokens = token_manager.identify_keyword(&tokens[0]).unwrap();

    println!("----------------------------------------------------");
    println!("Input: {}", input);
    println!("Returned tokens: {:#?}", split_tokens);

    match split_tokens[0].category {
        TokenCategory::Keyword(_) => assert!(true),
        _ => panic!("Expected Keyword, found {:?}", split_tokens[0].category)
    }
    assert_eq!(split_tokens[1].kind, number_kinds[0]);

    match split_tokens[2].category {
        TokenCategory::Keyword(_) => assert!(true),
        _ => panic!("Expected Keyword, found {:?}", split_tokens[0].category)
    }
    assert_eq!(split_tokens[3].kind, number_kinds[1]);
}

fn test_identify_keyword_combined(input: &str, attached_token_expected_kind: TokenKind) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    for token in token_manager.get_tokens().iter().filter(|t| t.value != " ") {
        let split_tokens = token_manager.identify_keyword(token).unwrap();

        println!("----------------------------------------------------");
        println!("Input: {}", input);
        println!("Returned tokens: {:#?}", split_tokens);

        match split_tokens[0].category {
            TokenCategory::Keyword(_) => assert!(true),
            _ => panic!("Expected Keyword, found {:?}", split_tokens[0].category)
        }
        assert_eq!(split_tokens[1].kind, attached_token_expected_kind);
    }
}

fn test_identify_keyword_combined_or_separated(input: &str) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    println!("----------------------------------------------------");
    println!("Input: {}", input);
    println!("Token: {:?}", &tokens[0]);

    let split_tokens = token_manager.identify_keyword(&tokens[0]).unwrap();
    println!("Returned tokens: {:#?}", split_tokens);

    match split_tokens[0].clone().category {
        TokenCategory::Keyword(keyword) => {
            match keyword.kind {
                KeywordKind::CombinedOrSeparated { .. } => assert!(true),
                KeywordKind::Separated { .. } => assert!(true),
                _ => panic!("Expected combined or separated keyword")
            }
        }
        _ => panic!("Expected Keyword, found {:?}", split_tokens[0].category)
    }
}


fn test_identify_keyword_ordinal_suffix(input: &str) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    println!("----------------------------------------------------");
    println!("Input: {}", input);

    let ret = token_manager.identify_keyword(&tokens[2]).unwrap();
    println!("Returned tokens: {:#?}", ret);

    match ret[0].clone().category {
        TokenCategory::Keyword(keyword) => {
            match keyword.kind {
                KeywordKind::OrdinalSuffix { .. } => assert!(true),
                _ => panic!("Expected ordinal suffix keyword")
            }
        }
        _ => panic!("Expected Keyword, found {:?}", ret[0].category)
    }
}

fn test_get_token_after(input: &str, index: usize, expected_value: &str) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    let ret = token_manager.get_token_after(index, true);
    println!("{:#?}", ret.clone().unwrap());
    assert_eq!(expected_value, ret.unwrap().value);
}

fn test_get_token_before(input: &str, index: usize, expected_value: &str) {
    let tokens = tokenizer::tokenize(input);
    let mut token_manager = TokenManager::new(tokens.clone());

    let ret = token_manager.get_token_before(index, true);
    println!("{:#?}", ret.clone().unwrap());
    assert_eq!(expected_value, ret.unwrap().value);
}
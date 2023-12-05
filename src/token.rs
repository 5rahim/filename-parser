#![allow(dead_code)]

use std::fmt::Display;

use uuid::Uuid;

use crate::keyword::{Keyword, KeywordCategory};
use crate::metadata::MetadataKind;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum BracketType {
    Opening, // e.g. [ ( {
    Closing, // e.g. ] ) }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenCategory {
    Bracket(BracketType), // e.g. [ ] ( ) { }
    Delimiter, // e.g. ' ', '_', '.'
    Separator, // e.g. - ~ +
    Keyword(Keyword), // e.g. Season prefix
    Unknown, // 🤷‍♂️
    Known(MetadataKind), // e.g. Anime Title,
    TokenParts(Vec<Token>) // e.g. S01E01
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum TokenKind {
    Unknown,
    SingleCharacter,
    String,
    Number, // "00", "01"
    NumberLike, // "01v2" "01'" "01x05" "01.5", "1st",
    Year,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub uuid: Uuid,
    pub value: String,
    pub enclosed: bool,
    pub category: TokenCategory,
    pub kind: TokenKind
}

impl Token {
    pub fn new<T: Display>(value: T, category: TokenCategory) -> Token {
        Token {
            uuid: Uuid::new_v4(),
            value: value.to_string(),
            enclosed: false,
            category,
            kind: TokenKind::Unknown,
        }
    }
    pub fn new_with_kind<T: Display>(value: T, category: TokenCategory, kind: TokenKind) -> Token {
        Token {
            uuid: Uuid::new_v4(),
            value: value.to_string(),
            enclosed: false,
            category,
            kind,
        }
    }

    pub fn is_enclosed(&self) -> bool {
        self.enclosed
    }

    pub fn has_category(&self, category: TokenCategory) -> bool {
        self.category == category
    }

    pub fn has_metadata_kind(&self, kind: MetadataKind) -> bool {
        if let Some(s_kind) = self.category.get_metadata_kind() {
            return s_kind == kind;
        }
        false
    }
}


//------


impl TokenCategory {

    pub fn get_metadata_kind(&self) -> Option<MetadataKind> {
        match self {
            TokenCategory::Known(kind) => Some(kind.to_owned()),
            _ => None
        }
    }

    pub fn is_bracket(&self) -> bool {
        match self {
            TokenCategory::Bracket(_) => true,
            _ => false
        }
    }
    pub fn is_delimiter(&self) -> bool {
        match self {
            TokenCategory::Delimiter => true,
            _ => false
        }
    }
    pub fn is_separator(&self) -> bool {
        match self {
            TokenCategory::Separator => true,
            _ => false
        }
    }
    pub fn is_any_keyword(&self) -> bool {
        match self {
            TokenCategory::Keyword(_) => true,
            _ => false
        }
    }
    pub fn is_keyword(&self, keyword_category: KeywordCategory) -> bool {
        match self {
            TokenCategory::Keyword(keyword) => {
                return keyword.category == keyword_category;
            },
            _ => false
        }
    }
    pub fn get_keyword(&self) -> Option<Keyword> {
        match self {
            TokenCategory::Keyword(keyword) => {
                return Some(keyword.clone())
            },
            _ => None
        }
    }
    pub fn is_token_parts(&self) -> bool {
        match self {
            TokenCategory::TokenParts(_) => true,
            _ => false
        }
    }
    pub fn get_token_parts(&self) -> Option<Vec<Token>> {
        match self {
            TokenCategory::TokenParts(tokens) => {
                return Some(tokens.clone())
            },
            _ => None
        }
    }
    pub fn is_unknown(&self) -> bool {
        match self {
            TokenCategory::Unknown => true,
            _ => false
        }
    }
    pub fn is_known(&self) -> bool {
        match self {
            TokenCategory::Known(_) => true,
            _ => false
        }
    }
    pub fn is_opening_bracket(&self) -> bool {
        match self {
            TokenCategory::Bracket(b_type) => {
                return *b_type == BracketType::Opening;
            },
            _ => false
        }
    }
    pub fn is_closing_bracket(&self) -> bool {
        match self {
            TokenCategory::Bracket(b_type) => {
                return *b_type == BracketType::Closing;
            },
            _ => false
        }
    }
}

impl TokenKind {
    pub fn is_string(&self) -> bool {
        match self {
            TokenKind::String => true,
            _ => false
        }
    }
    pub fn is_single_character(&self) -> bool {
        match self {
            TokenKind::SingleCharacter => true,
            _ => false
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            TokenKind::Number => true,
            _ => false
        }
    }
    pub fn is_number_like(&self) -> bool {
        match self {
            TokenKind::NumberLike => true,
            _ => false
        }
    }
    pub fn is_number_or_like(&self) -> bool {
        match self {
            TokenKind::Number => true,
            TokenKind::NumberLike => true,
            _ => false
        }
    }
}
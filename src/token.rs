#![allow(dead_code)]

use std::fmt::Display;

use uuid::Uuid;

use crate::keyword::{Keyword};
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

    pub fn get_bracket_type(self) -> Option<BracketType> {
        match self.category {
            TokenCategory::Bracket(n) => {
                Some(n)
            }
            _ => None
        }
    }
}
#![allow(dead_code)]

use crate::token::TokenKind;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum KeywordCategory {
    SeasonPrefix,
    EpisodePrefix,
    VolumePrefix,
    PartPrefix,
    AnimeType,
    VideoTerm,
    AudioTerm,
    DeviceCompat,
    FileExtension,
    Language,
    ReleaseInfo,
    ReleaseVersion,
    ReleaseGroup,
    Subtitles,
    Source,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum KeywordPriority {
    Normal, // Will override Unknown tokens
    Low // Low priority keywords will be parsed at the end (after other tokens have been analyzed)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum KeywordKind {
    Standalone, // e.g. MOVIE
    Combined { next_token_kind: TokenKind }, // Needs to be followed by a specific token to be valid. e.g. S01 or E01
    CombinedOrSeparated { next_token_kind: TokenKind }, // e.g. EP 01 or EP01
    Separated { next_token_kind: TokenKind }, // e.g. Season 01
    Suffix { previous_token_kind: TokenKind }, // e.g. 2nd Season TODO
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Keyword {
    pub value: String,
    pub category: KeywordCategory,
    pub kind: KeywordKind,
    pub priority: KeywordPriority
}

impl Keyword {

    pub fn new(value: String, category: KeywordCategory, kind: KeywordKind, priority: KeywordPriority) -> Keyword {
        Keyword {
            value,
            category,
            kind,
            priority,
        }
    }

    pub fn is_standalone(&self) -> bool {
        match self.kind {
            KeywordKind::Standalone => true,
            _ => false
        }
    }
    pub fn is_combined(&self) -> bool {
        match self.kind {
            KeywordKind::Combined { .. } => true,
            _ => false
        }
    }
    pub fn is_combined_or_separated(&self) -> bool {
        match self.kind {
            KeywordKind::CombinedOrSeparated { .. } => true,
            _ => false
        }
    }
    pub fn is_separated(&self) -> bool {
        match self.kind {
            KeywordKind::Separated { .. } => true,
            _ => false
        }
    }

}
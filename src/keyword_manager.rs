#![allow(dead_code)]

use crate::keyword::{Keyword, KeywordCategory, KeywordKind, KeywordPriority};
use crate::token::{TokenKind};

#[derive(Debug, Clone)]
pub struct KeywordManager {
    keywords: Vec<(String, Keyword)>,
}

impl KeywordManager {
    pub fn new() -> KeywordManager {
        let mut kwm = KeywordManager {
            keywords: vec![]
        };

        kwm
            // Seasons
            .add_group(
                KeywordCategory::SeasonPrefix,
                KeywordKind::Combined { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec!["S"])
            .add_group(
                KeywordCategory::SeasonPrefix,
                KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec!["SEASON", "SAISON", "SEASONS", "SAISONS"])
            .add_group(
                KeywordCategory::SeasonPrefix,
                KeywordKind::OrdinalSuffix,
                KeywordPriority::Normal,
                vec!["SEASON", "SAISON", "SEASONS", "SAISONS"])

            // Episodes
            .add_group(
                KeywordCategory::EpisodePrefix,
                KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec!["EPISODE", "EPISODE.", "EPISODES",
                     "CAPITULO", "EPISODIO", "FOLGE"])
            .add_group(
                KeywordCategory::EpisodePrefix,
                KeywordKind::Combined { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec!["E"])
            .add_group(
                KeywordCategory::EpisodePrefix,
                KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec!["EP", "EP.", "EPS", "EPS."])

            // Anime Type

            .add_group( // -> OVA 01
                        KeywordCategory::AnimeType,
                        KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                        KeywordPriority::Normal,
                        vec!["MOVIE", "OAD", "OAV", "ONA", "OVA", "SPECIAL", "SPECIALS", "ED", "ENDING", "NCED", "NCOP", "OPED", "OP", "OPENING",
                             "TV", "番外編", "總集編", "映像特典", "特典", "特典アニメ"])
            .add_group( // -> MOVIE, OVA
                        KeywordCategory::AnimeType,
                        KeywordKind::Standalone,
                        KeywordPriority::Normal,
                        vec!["MOVIE", "GEKIJOUBAN", "ONA", "OVA", "OAV", "OAD"])
            .add_group( //
                        KeywordCategory::AnimeType,
                        KeywordKind::Standalone,
                        KeywordPriority::Low,
                        vec!["ED", "ENDING", "NCED", "NCOP", "OPED", "OP", "OPENING", "PREVIEW",
                             "PV", "EVENT", "TOKUTEN", "LOGO", "CM", "SPOT", "MENU"])

            // Audio
            .add_group(
                KeywordCategory::AudioTerm,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![// Audio channels
                     "2.0CH", "2CH", "5.1", "5.1CH", "DTS", "DTS-ES", "DTS5.1", "TRUEHD5.1",
                     // Audio codec
                     "AAC", "AACX2", "AACX3", "AACX4", "AC3", "EAC3", "E-AC-3", "FLAC",
                     "FLACX2", "FLACX3", "FLACX4", "LOSSLESS", "MP3", "OGG", "VORBIS",
                     "DD2", "DD2.0",
                     // Audio language
                     "DUALAUDIO", "DUAL-AUDIO"])

            // Devices
            .add_group(
                KeywordCategory::DeviceCompat,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec!["IPAD3", "IPHONE5", "IPOD", "PS3", "XBOX", "XBOX360"])
            .add_group(
                KeywordCategory::DeviceCompat,
                KeywordKind::Standalone,
                KeywordPriority::Low,
                vec!["ANDROID"])

            .add_group(
                KeywordCategory::FileExtension,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "3GP", "AVI", "DIVX", "FLV", "M2TS", "MKV", "MOV", "MP4", "MPG",
                    "OGM", "RM", "RMVB", "TS", "WEBM", "WMV",
                ],
            )
            .add_group(
                KeywordCategory::FileExtension,
                KeywordKind::Standalone,
                KeywordPriority::Low,
                vec![
                    "AAC", "AIFF", "FLAC", "M4A", "MP3", "MKA", "OGG", "WAV", "WMA",
                    "7Z", "RAR", "ZIP", "ASS", "SRT",
                ],
            )

            // Languages
            .add_group(
                KeywordCategory::Language,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "ENG", "ENGLISH", "ESPANOL", "JAP", "PT-BR", "SPANISH", "VOSTFR",
                ],
            )
            .add_group(
                KeywordCategory::Language,
                KeywordKind::Standalone,
                KeywordPriority::Low,
                vec![
                    "ESP", "ITA",
                ],
            )

            // Release Information
            .add_group(
                KeywordCategory::ReleaseInfo,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "REMASTER", "REMASTERED", "UNCENSORED", "UNCUT", "TS", "VFR",
                    "WIDESCREEN", "WS", "BATCH", "COMPLETE", "PATCH", "REMUX",
                ],
            )

            .add_group(
                KeywordCategory::ReleaseInfo,
                KeywordKind::Standalone,
                KeywordPriority::Low,
                vec![
                    "END", "FINAL",
                ],
            )

            // Release Group
            .add_group(
                KeywordCategory::ReleaseGroup,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "THORA", "HORRIBLESUBS", "ERAI-RAWS", "SUBSPLEASE",
                ],
            )

            // Release Version
            .add_group(
                KeywordCategory::ReleaseVersion,
                KeywordKind::Standalone,
                KeywordPriority::Low,
                vec![
                    "V0", "V1", "V2", "V3", "V4",
                ],
            )

            // Source
            .add_group(
                KeywordCategory::Source,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "BD", "BDRIP", "BLURAY", "BLU-RAY", "DVD", "DVD5", "DVD9",
                    "DVD-R2J", "DVDRIP", "DVD-RIP", "R2DVD", "R2J", "R2JDVD",
                    "R2JDVDRIP", "HDTV", "HDTVRIP", "TVRIP", "TV-RIP",
                    "WEBCAST", "WEBRIP",
                ],
            )

            .add_group(
                KeywordCategory::Subtitles,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    "ASS", "BIG5", "DUB", "DUBBED", "HARDSUB", "HARDSUBS", "RAW",
                    "SOFTSUB", "SOFTSUBS", "SUB", "SUBBED", "SUBTITLED", "MULTISUB",
                ],
            )

            // Video Terms
            .add_group(
                KeywordCategory::VideoTerm,
                KeywordKind::Standalone,
                KeywordPriority::Normal,
                vec![
                    // Frame rate
                    "23.976FPS", "24FPS", "29.97FPS", "30FPS", "60FPS", "120FPS",
                    // Video codec
                    "8BIT", "8-BIT", "10BIT", "10BITS", "10-BIT", "10-BITS",
                    "HI10", "HI10P", "HI444", "HI444P", "HI444PP",
                    "H264", "H265", "H.264", "H.265", "X264", "X265", "X.264",
                    "AVC", "HEVC", "HEVC2", "DIVX", "DIVX5", "DIVX6", "XVID",
                    "AV1",
                    "HDR", "DV", "DOLBY VISION",
                    // Video format
                    "AVI", "RMVB", "WMV", "WMV3", "WMV9",
                    // Video quality
                    "HQ", "LQ",
                    // Video resolution
                    "HD", "SD", "4K",
                ],
            )

            // Volume Prefix
            .add_group(
                KeywordCategory::VolumePrefix,
                KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec![
                    "VOL", "VOL.", "VOLUME", "VOLUMES",
                ],
            )
            // Volume Prefix
            .add_group(
                KeywordCategory::PartPrefix,
                KeywordKind::CombinedOrSeparated { next_token_kind: TokenKind::NumberLike },
                KeywordPriority::Normal,
                vec![
                    "PART", "PT.",
                ],
            );


        kwm
    }

    fn add_group(
        &mut self,
        keyword_category: KeywordCategory,
        keyword_kind: KeywordKind,
        keyword_priority: KeywordPriority,
        values: Vec<&str>,
    ) -> &mut Self {
        values.iter().for_each(|value| {
            self.keywords.push(
                (
                    value.to_string(),
                    Keyword::new(value.to_string(), keyword_category, keyword_kind, keyword_priority)
                ),
            );
        });
        self
    }

    fn add(&mut self, value: &str, keyword: Keyword) -> &mut Self {
        self.keywords.push((value.to_string(), keyword));
        self
    }

    //-----------

    /// Find a keyword by value.
    pub fn find_standalone(&self, token_value: &str) -> Option<Keyword> {
        self.keywords
            .iter()
            .filter(|(_, val)| val.is_standalone())
            .find(|(key, _)| key.to_uppercase() == token_value.to_uppercase())
            .map(|(_, value)| value.clone())
    }

    /// Find a keyword by value.
    pub fn find(&self, token_value: &str) -> Option<Keyword> {
        let keyword = self.find_standalone(token_value);
        let uppercase_value = token_value.to_uppercase();
        match keyword {
            Some(keyword) => Some(keyword),
            None => {
                // Get keywords that are Combined or CombinedOrSeparated
                let combined_or_separated_keywords = self.get_combined_or_separated_keywords();

                // Find the keyword with the longest key
                match combined_or_separated_keywords
                    .iter()
                    .filter(|(key, _val)| uppercase_value.starts_with(key))
                    .max_by_key(|(key, _val)| key.len())
                {
                    Some((_, longest_keyword)) => Some(longest_keyword.clone()),
                    None => None
                }
            }
        }
    }

    /// Find a keyword by value.
    pub fn find_many(&self, token_value: &str) -> Option<Vec<Keyword>> {
        // Get keywords that are Combined or CombinedOrSeparated
        let filtered: Vec<Keyword> = self.keywords
            .iter().cloned()
            .filter(|(key, _val)| token_value.to_uppercase().starts_with(key))
            .map(|(_, val)| val)
            .collect();

        return Some(filtered);
    }


    //-----------

    /// Get standalone keywords
    fn get_standalone_keywords(&self) -> Vec<(String, Keyword)> {
        return self.keywords
            .clone()
            .into_iter()
            .filter(|(_key, val)| { val.is_standalone() })
            .collect();
    }

    /// Get keywords that require to be combined or separated
    fn get_combined_or_separated_keywords(&self) -> Vec<(String, Keyword)> {
        return self.keywords
            .clone()
            .into_iter()
            .filter(|(_key, val)| { val.is_combined() || val.is_combined_or_separated() || val.is_ordinal_suffix() })
            .collect();
    }
}

//--------------------------------------------------------------------------------------------------

#[test]
fn test_new_keyword_manager() {
    let keyword_manager = KeywordManager::new();

    println!("{:#?}", keyword_manager);

    assert!(true);
}

#[test]
fn test_find_01() {
    let keyword_manager = KeywordManager::new();

    let ret = keyword_manager.find("MOVIE");

    println!("{:#?}", ret);

    if let Some(keyword) = ret {
        assert_eq!(String::from("MOVIE"), keyword.value);
    } else {
        panic!("No keyword found")
    }
}

#[test]
fn test_find_02() {
    let keyword_manager = KeywordManager::new();

    let ret = keyword_manager.find("S01");
    let ret2 = keyword_manager.find("EP01");
    let ret3 = keyword_manager.find("OP1");

    println!("{:#?}", ret);
    println!("{:#?}", ret2);
    println!("{:#?}", ret3);

    assert_eq!(String::from("S"), ret.unwrap().value);
    assert_eq!(String::from("EP"), ret2.unwrap().value);
    assert_eq!(String::from("OP"), ret3.unwrap().value);
}

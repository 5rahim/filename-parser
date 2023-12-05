// example
// <filename> ::= <release_group> <anime_title> <anime_episode_group> <file_metadata> <hash>
// <release_group> ::= "[" ID "]" | e
// <anime_title> ::= <mostly_latin_string> | "[" ID "]" <release_group> | e
// <anime_episode_group> ::= <season_and_episode> <episode_title> | <season> <episode> | e
// <episode_title> ::= <mostly_latin_string> | e
// <mostly_latin_string> ::= <word>+
// <season_and_episode> ::= <season> <episode> | "S" <number> "Ep?" <episode_number> | <number> "x" <episode_number> | e
// <season> ::= "S" <number> <part> | <season_range> | e
// <part> ::= "Part" <number> | "Cour" <number>
// <season_range> ::= "S" <number> <range_sep> <number> | e
// <episode> ::= <non_main_id> "Ep?" <episode_number> | <episode_number> <range_sep> <episode_number> | e
// <episode_range> ::= <range_sep> <episode_number> | e
// <episode_number> ::= <number> <versioning>
// <versioning> ::= "v" <number> | "'" | "." <number> | e
// <file_metadata> ::= <metadata_keyword>+
// <metadata_keyword> ::= "[" KEYWORD "]" | KEYWORD | e
// <non_main_id> ::= "OVA" | "ED" | ... | e
// <number> ::= digit+
// <hash> ::=
// <range_sep> ::= "-" | "~" | "to"

#![allow(dead_code)]

use std::path::Path;

use crate::token::{BracketType, Token, TokenCategory};

pub static DELIMITERS: [char; 5] = ['_', ' ', '　', '.', '|'];
pub static SEPARATORS: [char; 3] = ['-', '+', '~'];
pub static OPENING_BRACKETS: [char; 7] = ['[', '(', '{', '\u{300C}', '\u{300E}', '\u{3011}', '\u{FF08}'];
pub static CLOSING_BRACKETS: [char; 7] = [']', ')', '}', '\u{300D}', '\u{300F}', '\u{3010}', '\u{FF09}'];


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars();

    let mut raw_tokens: Vec<Token> = vec![];

    while let Some(char) = chars.next() {
        let category = match char {
            _ if OPENING_BRACKETS.contains(&char) => TokenCategory::Bracket(BracketType::Opening),
            _ if CLOSING_BRACKETS.contains(&char) => TokenCategory::Bracket(BracketType::Closing),
            _ if DELIMITERS.contains(&char) => TokenCategory::Delimiter,
            _ if SEPARATORS.contains(&char) => TokenCategory::Separator,
            _ => TokenCategory::Unknown
        };
        raw_tokens.push(Token::new(&char, category))
    }

    let mut tokens = fuse_unknown_tokens(raw_tokens);

    find_enclosed_tokens(&mut tokens);

    tokens
}

fn find_enclosed_tokens(tokens: &mut Vec<Token>) {
    let mut opened = false;
    let mut enclosed_buffer: Vec<&mut Token> = vec![];

    for token in tokens.iter_mut() {
        // Found '['
        if token.category.is_opening_bracket() {
            opened = true;
        }
        // Look for TokenCategory::Unknown before BracketType::Closing
        if opened {
            match token.category {
                TokenCategory::Unknown => {
                    enclosed_buffer.push(token);
                }
                TokenCategory::Bracket(BracketType::Closing) => {
                    enclosed_buffer.push(token);
                    opened = false;
                }
                _ => {}
            }
        }
    }

    // If it's still opened (last item is not the closing bracket)
    if let Some(last_item) = enclosed_buffer.last() {
        match last_item.category {
            TokenCategory::Unknown => {
                if let Some(_) = enclosed_buffer.pop() {
                    // last item removed
                }
            }
            _ => {}
        }
    }

    for token in enclosed_buffer {
        if token.category == TokenCategory::Unknown {
            token.enclosed = true;
        }
    }
}

// e.g. '[','a','b','c',']' -> '[','abc',']'
fn fuse_unknown_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut fused_tokens = Vec::new();
    let mut unknown_buffer = String::new();
    let mut is_unknown_sequence = false;

    for token in tokens {
        match token.category {
            TokenCategory::Unknown => {
                if is_unknown_sequence {
                    // Continue building the unknown sequence
                    unknown_buffer.push_str(&token.value);
                } else {
                    // Start a new unknown sequence
                    unknown_buffer = token.value.clone();
                    is_unknown_sequence = true;
                }
            }
            _ => {
                // End of the unknown sequence
                if is_unknown_sequence {
                    fused_tokens.push(Token::new(unknown_buffer.clone(), TokenCategory::Unknown));
                    is_unknown_sequence = false;
                }
                fused_tokens.push(token);
            }
        }
    }

    // Handle the case where the last token is unknown
    if is_unknown_sequence {
        fused_tokens.push(Token::new(unknown_buffer, TokenCategory::Unknown));
    }

    fused_tokens
}


pub fn remove_file_extension(input: &str) -> String {
    if let Some(stem) = Path::new(input).file_stem() {
        if let Some(str_stem) = stem.to_str() {
            return str_stem.into();
        }
    }
    return input.into();
}

//---------------

#[test]
fn test_enclosed() {
    let input = String::from("[HorribleSubs]");

    let tokens = tokenize(&input);

    println!("{:#?}", tokens);

    assert_eq!(String::from("HorribleSubs"), tokens[1].value);
    assert!(tokens[1].is_enclosed());
}

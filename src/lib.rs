mod tokenizer;
mod keyword;
mod token;
mod token_manager;
mod keyword_manager;
mod metadata;
mod parser;
mod token_helper;
mod utils;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_extension() {
        let input = String::from("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv");

        let input_without_extension = tokenizer::remove_file_extension(&input);

        println!("{:?}", input_without_extension);

        assert_eq!(String::from("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p]"), input_without_extension)
    }

    #[test]
    fn test_random() {
        let input = String::from("[HorribleSubs]_Tower_of_Druaga_-_Sword_of_Uruk_-_04+Movie_[480p].mkv");

        let input_without_extension = tokenizer::remove_file_extension(&input);

        let tokens = tokenizer::tokenize(&input_without_extension);

        println!("{:#?}", tokens);

        assert!(true)

    }

    // Add more test cases as needed
}

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use clap::{Arg, Command};
use std::env;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

mod scripts;
mod types;

#[derive(Debug)]
pub struct UserInputContext {
    filename: String,
    input: Option<scripts::ScriptName>,
    output: Option<scripts::ScriptName>,
    input_char_lengths: Vec<usize>,
}

#[derive(Debug)]
pub struct Text {
    original: Vec<char>,
    intermediate_representation: Vec<
        types::Token<
            types::Vowels,
            types::VowelMarks,
            types::Accents,
            types::Consonants,
            types::Yogavāhas,
            types::Symbols,
            types::Virāma,
        >,
    >,
    transliterated: String,
}

pub fn generate_scriptname_from_arg(user_input: &str) -> Option<scripts::ScriptName> {
    match user_input {
        "devanagari" => Some(scripts::ScriptName::Devanagari),
        "telugu" => Some(scripts::ScriptName::Telugu),
        "iast_iso" => Some(scripts::ScriptName::IastIso),
        _ => None,
    }
}

fn create_context() -> UserInputContext {
    let args = Command::new("transliterator")
        .arg(Arg::new("filename").required(true))
        .arg(Arg::new("input_script").required(true))
        .arg(Arg::new("output_script").required(true))
        .get_matches();
    let input_script: String = args
        .get_one::<String>("input_script")
        .expect("input script required")
        .clone();
    let output_script: String = args
        .get_one::<String>("output_script")
        .expect("output script required")
        .clone();
    let filename: String = args
        .get_one::<String>("filename")
        .expect("output script required")
        .clone();
    let mut context = UserInputContext {
        filename: filename,
        input: None,
        output: None,
        input_char_lengths: vec![],
    };
    context.input = generate_scriptname_from_arg(&input_script);
    context.output = generate_scriptname_from_arg(&output_script);
    let filename: String = args
        .get_one::<String>("filename")
        .expect("filename required")
        .clone()
        .to_string();
    context
}

fn create_test_context() -> UserInputContext {
    let input_script: String = "devanagari".to_string();
    let output_script: String = "iast_iso".to_string();
    let mut context = UserInputContext {
        filename: "".to_string(),
        input: None,
        output: None,
        input_char_lengths: vec![],
    };
    context.input = generate_scriptname_from_arg(&input_script);
    context.output = generate_scriptname_from_arg(&output_script);
    context
}

pub fn read_file(filename: &str) -> Text {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(code) => code,
        Err(e) => panic!("{:?}", e),
    };
    let mut contents_vector: Vec<char> = Vec::new();
    let mut raw_contents: Vec<&str> =
        UnicodeSegmentation::graphemes(contents.as_str(), true).collect::<Vec<&str>>();
    for strs in raw_contents.iter_mut() {
        contents_vector.append(&mut strs.chars().collect::<Vec<char>>());
    }
    Text {
        original: contents_vector,
        intermediate_representation: vec![],
        transliterated: "".to_string(),
    }
}

pub fn read_string(contents: String) -> Text {
    let mut contents_vector: Vec<char> = Vec::new();
    let mut raw_contents: Vec<&str> =
        UnicodeSegmentation::graphemes(contents.as_str(), true).collect::<Vec<&str>>();
    for strs in raw_contents.iter_mut() {
        contents_vector.append(&mut strs.chars().collect::<Vec<char>>());
    }
    Text {
        original: contents_vector,
        intermediate_representation: vec![],
        transliterated: "".to_string(),
    }
}

fn char_to_token(chr: char, mapping: &scripts::TokenMap) -> types::TokensAggregated {
    let vowel = mapping.Vowels.get(&chr);
    let vowelmarks = mapping.VowelMarks.get(&chr);
    let accents = mapping.Accents.get(&chr);
    let consonants = mapping.Consonants.get(&chr);
    let yogavahas = mapping.Yogavāhas.get(&chr);
    let viramam = mapping.Virāmam.get(&chr);
    println!("{:?}", vowel);
    if !vowel.is_none() {
        vowel.unwrap().clone()
    } else if !vowelmarks.is_none() {
        vowelmarks.unwrap().clone()
    } else if !accents.is_none() {
        accents.unwrap().clone()
    } else if !consonants.is_none() {
        consonants.unwrap().clone()
    } else if !yogavahas.is_none() {
        yogavahas.unwrap().clone()
    } else if !viramam.is_none() {
        viramam.unwrap().clone()
    } else {
        types::TokensAggregated::Unk(chr)
    }
}

fn clean_up_vowel_to_vowelmarks() {}

fn txt_to_tokens(text: &mut Text, mapping: &scripts::TokenMap) {
    let mut tokens = vec![];
    for c in text.original.iter() {
        let t = char_to_token(*c, mapping);
        tokens.push(t);
    }
    text.intermediate_representation = tokens;
}

fn main() {
    let context = create_context();
    let mut text = read_file(context.filename.as_str());
    let tokenmapping = scripts::mapping_character_to_token(&context.input.unwrap());
    txt_to_tokens(&mut text, &tokenmapping);
    println!("{:?}", text.intermediate_representation);
}

#[cfg(test)]
mod tests {
    use crate::{Text, txt_to_tokens, read_string, create_test_context};
    use crate::types::TokensAggregated;
    use crate::types::{Consonants, VowelMarks};
    use crate::scripts::mapping_character_to_token;

    fn test(src: &mut Text, trg: Vec<TokensAggregated>) {
        let context = create_test_context();
        let tokenmapping = mapping_character_to_token(&context.input.unwrap());

        txt_to_tokens(src, &tokenmapping);
        assert_eq!(trg, src.intermediate_representation);
        for i in 0..trg.len(){
            assert_eq!(trg[i], src.intermediate_representation[i]);
        }
    }

    #[test]
    fn one() {
        let s = "सरस्वत".to_string();
        let mut s = read_string(s);
        let t: Vec<TokensAggregated> = vec![
            TokensAggregated::Consonant(Consonants::s),
            TokensAggregated::VowelMark(VowelMarks::a),
            TokensAggregated::Consonant(Consonants::r),
            TokensAggregated::VowelMark(VowelMarks::a),
            TokensAggregated::Consonant(Consonants::s),
            TokensAggregated::Consonant(Consonants::v),
            TokensAggregated::VowelMark(VowelMarks::a),
            TokensAggregated::Consonant(Consonants::t),
            TokensAggregated::VowelMark(VowelMarks::ī),
        ];
        test(&mut s, t);
    }
}

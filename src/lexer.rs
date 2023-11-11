/*
 * A Lexer takes a stream of characters and produces tokens. For our purposes,
 * we have mappings from devanagari out to our input script. As such, we will need
 * to invert such mappings since we need a map from our input out to devanagari.
 *
 * The second step then maps devanagari to our tokens.
 */

use crate::types::{RawScriptMap, Script, ScriptMap, Token};
use std::collections::HashMap;
use toml;

const TOKEN_TYPES: &[&str] = &[
    "vowels",
    "vowel_marks",
    "consonants",
    "yogavaahas",
    "virama",
    "candra",
    "symbols",
    "accents",
];

pub fn read_script(which: Script) -> RawScriptMap {
    let script = match which {
        Script::Telugu => include_str!("../maps/brahmic/telugu.toml"),
        Script::IastIso => include_str!("../maps/roman/iast_iso_m.toml"),
        Script::Devanagari => include_str!("../maps/brahmic/devanagari.toml"),
    };
    toml::from_str(script).unwrap()
}

pub fn produce_scriptmap_group(
    script: &RawScriptMap,
    group_name: String,
    invert: bool,
) -> HashMap<String, String> {
    let group = match script.get(&group_name) {
        Some(x) => x.clone(),
        None => HashMap::new(),
    };

    let mut map = HashMap::new();
    for (key, value) in group.iter() {
        let value = value.to_string().replace(&['\"'], "");
        if invert {
            map.insert(value, key.clone());
        } else {
            map.insert(key.clone(), value);
        }
    }

    map
}

pub fn produce_scriptmap(script: &RawScriptMap, invert: bool) -> ScriptMap {
    let mut groups: ScriptMap = HashMap::from([]);
    for token_type in TOKEN_TYPES {
        let scriptmap_group = produce_scriptmap_group(script, token_type.to_string(), invert);
        if !scriptmap_group.is_empty() {
        groups.insert(
            token_type.to_string(), scriptmap_group
        );
    }
    }
    if !groups.contains_key("vowel_marks") {
        groups.insert(
            "vowel_marks".to_string(),
            groups.get("vowels").unwrap().clone()
        );
    }
    groups
}

pub fn deva_to_enum(input: String) -> Token {
    match input.as_str() {
        "अ" => Token::a,
        "आ" => Token::A,
        "इ" => Token::i,
        "ई" => Token::I,
        "उ" => Token::u,
        "ऊ" => Token::U,
        "ऋ" => Token::RRi,
        "ॠ" => Token::RRI,
        "ऌ" => Token::LLi,
        "ॡ" => Token::LLI,
        "ऎ" => Token::e,
        "ए" => Token::E,
        "ऐ" => Token::ai,
        "ऒ" => Token::o,
        "ओ" => Token::O,
        "औ" => Token::au,
        "क" => Token::k,
        "ख" => Token::kh,
        "ग" => Token::g,
        "घ" => Token::gh,
        "ङ" => Token::ng,
        "च" => Token::c,
        "छ" => Token::ch,
        "ज" => Token::j,
        "झ" => Token::jh,
        "ञ" => Token::ny,
        "ट" => Token::T,
        "ठ" => Token::Th,
        "ड" => Token::D,
        "ढ" => Token::Dh,
        "ण" => Token::N,
        "त" => Token::t,
        "थ" => Token::th,
        "द" => Token::d,
        "ध" => Token::dh,
        "न" => Token::n,
        "प" => Token::p,
        "फ" => Token::ph,
        "ब" => Token::b,
        "भ" => Token::bh,
        "म" => Token::m,
        "य" => Token::y,
        "र" => Token::r,
        "ल" => Token::l,
        "व" => Token::v,
        "श" => Token::sh,
        "ष" => Token::Sh,
        "स" => Token::s,
        "ह" => Token::h,
        "ा" => Token::vm_A,
        "ि" => Token::vm_i,
        "ी" => Token::vm_I,
        "ु" => Token::vm_u,
        "ू" => Token::vm_U,
        "ृ" => Token::vm_RRi,
        "ॄ" => Token::vm_RRI,
        "ॢ" => Token::vm_LLi,
        "ॣ" => Token::vm_LLI,
        "ॆ" => Token::vm_e,
        "े" => Token::vm_E,
        "ै" => Token::vm_ai,
        "ॊ" => Token::vm_o,
        "ो" => Token::vm_O,
        "ौ" => Token::vm_au,
        "ँ" => Token::cbindu__y,
        "ं" => Token::M,
        "ः" => Token::H,
        "्" => Token::virama,
        "॑" => Token::svarita,
        "\u{952}" => Token::anudaatta,
        "।" => Token::danda,
        "॥" => Token::dvidanda,
        _ => Token::Unknown(input),
    }
}

impl Token {
    pub fn to_devanagari(&self) -> String {
        enum_to_deva(self)
    }
}

fn enum_to_deva(token: &Token) -> String {
    match token {
        Token::a => "अ",
        Token::A => "आ",
        Token::i => "इ",
        Token::I => "ई",
        Token::u => "उ",
        Token::U => "ऊ",
        Token::RRi => "ऋ",
        Token::RRI => "ॠ",
        Token::LLi => "ऌ",
        Token::LLI => "ॡ",
        Token::e => "ऎ",
        Token::E => "ए",
        Token::ai => "ऐ",
        Token::o => "ऒ",
        Token::O => "ओ",
        Token::au => "औ",
        Token::k => "क",
        Token::kh => "ख",
        Token::g => "ग",
        Token::gh => "घ",
        Token::ng => "ङ",
        Token::c => "च",
        Token::ch => "छ",
        Token::j => "ज",
        Token::jh => "झ",
        Token::ny => "ञ",
        Token::T => "ट",
        Token::Th => "ठ",
        Token::D => "ड",
        Token::Dh => "ढ",
        Token::N => "ण",
        Token::t => "त",
        Token::th => "थ",
        Token::d => "द",
        Token::dh => "ध",
        Token::n => "न",
        Token::p => "प",
        Token::ph => "फ",
        Token::b => "ब",
        Token::bh => "भ",
        Token::m => "म",
        Token::y => "य",
        Token::r => "र",
        Token::l => "ल",
        Token::v => "व",
        Token::sh => "श",
        Token::Sh => "ष",
        Token::s => "स",
        Token::h => "ह",
        Token::vm_A => "ा",
        Token::vm_i => "ि",
        Token::vm_I => "ी",
        Token::vm_u => "ु",
        Token::vm_U => "ू",
        Token::vm_RRi => "ृ",
        Token::vm_RRI => "ॄ",
        Token::vm_LLi => "ॢ",
        Token::vm_LLI => "ॣ",
        Token::vm_e => "ॆ",
        Token::vm_E => "े",
        Token::vm_ai => "ै",
        Token::vm_o => "ॊ",
        Token::vm_O => "ो",
        Token::vm_au => "ौ",
        Token::cbindu__y => "ँ",
        Token::M => "ं",
        Token::H => "ः",
        Token::virama => "्",
        Token::svarita => "॑",
        Token::anudaatta => "॒",
        Token::danda => "।",
        Token::dvidanda => "॥",
        _ => " _unk_ ",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map_search(search: &str, group: &str, result: &str) {
        let script = read_script(Script::Devanagari);
        let group = match script.get(group) {
            Some(x) => x,
            _ => panic!("Issue loading vowels from devanagari script."),
        };

        let value = match group.get(search) {
            Some(toml::Value::String(x)) => x,
            _ => panic!("Issue loading अ from devanagari script\'s vowels."),
        };

        assert_eq!(value, result);
    }

    #[test]
    fn test_read_script() {
        map_search("अ", "vowels", "अ");
        map_search("क", "consonants", "क");
    }

    #[test]
    fn test_scriptmap_group_inversion() {
        let mut script = read_script(Script::Telugu);
        let inverted = produce_scriptmap_group(&mut script, "vowels".to_string(), true);
        println!("{:?}", inverted);
        assert_eq!(inverted.get("అ").unwrap(), "अ");
    }

    #[test]
    fn test_produce_scriptmap() {
        let mut script = read_script(Script::Telugu);
        let map = produce_scriptmap(&mut script, false);
        println!("{:?}", map);
        assert_eq!(map.get("vowels").unwrap().get("अ").unwrap(), "అ");
    }

    #[test]
    fn test_deva_to_enum() {
        for i in vec![("अ", Token::a), ("क", Token::k)].iter() {
            assert_eq!(deva_to_enum(i.0.to_string()), i.1);
        }
    }

    #[test]
    fn test_enum_to_deva() {
        for (token, deva) in vec![(Token::a, "अ"), (Token::k, "क")].iter() {
            assert_eq!(token.to_devanagari(), deva.to_string());
        }
    }
}

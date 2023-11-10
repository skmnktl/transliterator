/*
 * A Lexer takes a stream of characters and produces tokens. For our purposes,
 * we have mappings from devanagari out to our input script. As such, we will need
 * to invert such mappings since we need a map from our input out to devanagari.
 *
 * The second step then maps devanagari to our tokens.
 */

use crate::types::{RawScriptMap, ScriptMap, ScriptName, Token};
use std::collections::HashMap;
use toml;

pub static CHAR_GROUPS: [&str; 9] = [
    "vowels",
    "vowel_marks",
    "consonants",
    "yogavaahas",
    "virama",
    "candra",
    "symbols",
    "placeholders",
    "accents",
];

pub fn read_script(which: &ScriptName) -> RawScriptMap {
    let script = match *which {
        ScriptName::Telugu => include_str!("../maps/brahmic/telugu.toml"),
        ScriptName::IastIso => include_str!("../maps/roman/iast_iso_m.toml"),
        ScriptName::Devanagari => include_str!("../maps/brahmic/devanagari.toml"),
    };
    let mut script: RawScriptMap = toml::from_str(script).unwrap();
    script
}

pub fn produce_scriptmap_group(
    script: &mut RawScriptMap,
    group_name: String,
    invert: bool,
) -> HashMap<String, String> {
    if group_name == "placeholders".to_string() {
        return HashMap::from([
            (" ".to_string(), " ".to_string()),
            ("\n".to_string(), "\n".to_string()),
        ]);
    }
    let group = match script.get(&group_name) {
        Some(x) => x.clone(),
        None => HashMap::from([]),
    };
    let mut map: HashMap<String, String> = HashMap::from([]);

    for k in group.keys() {
        let v = group.get(k).clone().unwrap().to_string();
        let v = v.replace(&['\"'], "");
        if invert {
            map.insert(v, k.clone());
        } else {
            map.insert(k.clone(), v);
        }
    }

    map
}

pub fn produce_scriptmap(script: &mut RawScriptMap, invert: bool) -> ScriptMap {
    let mut groups: ScriptMap = HashMap::from([]);
    for group_name in CHAR_GROUPS {
        groups.insert(
            group_name.to_string(),
            produce_scriptmap_group(script, group_name.to_string(), invert),
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
        "ा" => Token::A,
        "ि" => Token::i,
        "ी" => Token::I,
        "ु" => Token::u,
        "ू" => Token::U,
        "ृ" => Token::RRi,
        "ॄ" => Token::RRI,
        "ॢ" => Token::LLi,
        "ॣ" => Token::LLI,
        "ॆ" => Token::e,
        "े" => Token::E,
        "ै" => Token::ai,
        "ॊ" => Token::o,
        "ो" => Token::O,
        "ौ" => Token::au,
        "ँ" => Token::cbindu,
        "ं" => Token::M,
        "ः" => Token::H,
        "्" => Token::virama,
        "॑" => Token::svarita,
        "\u{952}" => Token::anudaatta,
        "।" => Token::danda,
        "॥" => Token::dvidanda,
        " " => Token::whitespace,
        "\n" => Token::newline,
        _ => Token::unk,
    }
}

pub fn enum_to_deva(token: Token) -> String {
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
        Token::A => "ा",
        Token::i => "ि",
        Token::I => "ी",
        Token::u => "ु",
        Token::U => "ू",
        Token::RRi => "ृ",
        Token::RRI => "ॄ",
        Token::LLi => "ॢ",
        Token::LLI => "ॣ",
        Token::e => "ॆ",
        Token::E => "े",
        Token::ai => "ै",
        Token::o => "ॊ",
        Token::O => "ो",
        Token::au => "ौ",
        Token::cbindu => "ँ",
        Token::M => "ं",
        Token::H => "ः",
        Token::virama => "्",
        Token::svarita => "॑",
        Token::anudaatta => "॒",
        Token::danda => "।",
        Token::dvidanda => "॥",
        Token::whitespace => " ",
        Token::newline => "\n",
        Token::unk => " _unk_ ",
        _ => " _unk_ ",
    }
    .to_string()
}

pub fn map_vowel_marks(token: Token) -> String {
    match token {
        Token::a => "",
        Token::A => "ा",
        Token::i => "ि",
        Token::I => "ी",
        Token::u => "ु",
        Token::U => "ू",
        Token::RRi => "ृ",
        Token::RRI => "ॄ",
        Token::LLi => "ॢ",
        Token::LLI => "ॣ",
        Token::e => "ॆ",
        Token::E => "े",
        Token::ai => "ै",
        Token::o => "ॊ",
        Token::O => "ो",
        Token::au => "ौ",
        _ => "_unk ",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map_search(search: &str, group: &str, result: &str) {
        let script = read_script(&ScriptName::Devanagari);
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
        let mut script = read_script(&ScriptName::Telugu);
        let inverted = produce_scriptmap_group(&mut script, "vowels".to_string(), true);
        println!("{:?}", inverted);
        assert_eq!(inverted.get("అ").unwrap(), "अ");
    }

    #[test]
    fn test_produce_scriptmap() {
        let mut script = read_script(&ScriptName::Telugu);
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
        for i in vec![(Token::a, "अ"), (Token::k, "क")].iter() {
            assert_eq!(enum_to_deva(i.0.clone()), i.1.to_string());
        }
    }
}

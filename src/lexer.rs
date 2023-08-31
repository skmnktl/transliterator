/*
 * A Lexer takes a stream of characters and produces tokens. For our purposes,
 * we have mappings from devanagari out to our input script. As such, we will need
 * to invert such mappings since we need a map from our input out to devanagari.
 *
 * The second step then maps devanagari to our tokens.
 */

use crate::types::{ScriptMap, ScriptName};
use std::collections::HashMap;
use toml;

pub fn read_script(which: &ScriptName) -> ScriptMap {
    let script = match *which {
        ScriptName::Telugu => include_str!("../maps/brahmic/telugu.toml"),
        ScriptName::IastIso => include_str!("../maps/roman/iast_iso_m.toml"),
        ScriptName::Devanagari => include_str!("../maps/brahmic/devanagari.toml"),
    };
    let script: ScriptMap = toml::from_str(script).unwrap();
    script
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
            _ => panic!("Issue loading अ from devanagari script's vowels."),
        };

        assert_eq!(value, result);
    }

    #[test]
    fn test_load_devanagari() {
        map_search("अ", "vowels", "अ");
        map_search("ँ", "yogavaahas", "ँ");
    }
}

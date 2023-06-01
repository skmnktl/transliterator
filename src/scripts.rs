use std::collections::{HashMap};
use toml;
use crate::types;

#[derive(Debug, Clone)]
pub enum ScriptName {
    Telugu,
    IastIso,
    Devanagari,
}

type ScriptMap = HashMap<String, HashMap<String, toml::Value>>;

pub fn read_script(which: &ScriptName) -> ScriptMap {
    let script = match *which {
        ScriptName::Telugu => include_str!("../common_maps/brahmic/telugu.toml"),
        ScriptName::IastIso => include_str!("../common_maps/roman/iast_iso_m.toml"),
        ScriptName::Devanagari => include_str!("../common_maps/brahmic/devanagari.toml")
    };
    let script: ScriptMap = toml::from_str(script).unwrap();
    script
}


pub fn script_to_deva(which: &ScriptName) -> ScriptMap{
    
    let script = read_script(which);
    let mut inverted = ScriptMap::new();
    let mut sub = HashMap::<String, toml::Value>::new();
    for i in ["vowels","vowel_marks", "yogavaahas","virama","accents", "consonants", "symbols"].iter(){
        let category = script.get(&i.to_string());
        if category.is_none(){
            continue;
        }
        for (k,v) in category.unwrap().iter(){
            if v.is_str(){
                let new_key = v.to_string().replace("\"","");
                sub.insert(new_key,toml::Value::String(k.clone().to_string()));
            }
        }
        inverted.insert(i.to_string(), sub.clone());
        sub = HashMap::<String, toml::Value>::new();
        
    }
    inverted
}

#[derive(Debug)]
pub struct TokenMap{
    pub Vowels: HashMap<char, types::TokensAggregated>,
    pub VowelMarks: HashMap<char, types::TokensAggregated>,
    pub Accents: HashMap<char, types::TokensAggregated>,
    pub Consonants: HashMap<char, types::TokensAggregated>,
    pub Yogavāhas: HashMap<char, types::TokensAggregated>,
    pub Symbols: HashMap<char, types::TokensAggregated>,
    pub Virāmam: HashMap<char, types::TokensAggregated>
}

impl TokenMap{
    pub fn new() -> TokenMap{
        TokenMap{
            Vowels: HashMap::<char, types::TokensAggregated>::new(), 
            VowelMarks: HashMap::<char, types::TokensAggregated>::new(),
            Accents: HashMap::<char, types::TokensAggregated>::new(),
            Consonants: HashMap::<char, types::TokensAggregated>::new(),
            Yogavāhas: HashMap::<char, types::TokensAggregated>::new(),
            Symbols: HashMap::<char, types::TokensAggregated>::new(),
            Virāmam: HashMap::<char, types::TokensAggregated>::new()
        }
    } 
}

pub fn mapping_character_to_token(which: &ScriptName) -> TokenMap{
    let script = script_to_deva(&which);
    let mut map_to_tokens = TokenMap::new();

    for (category, mapping) in script.iter(){
        for (c, value) in mapping.iter(){
            let chr_vec = c.to_string().chars().collect::<Vec<char>>();
            let val_vec = value.to_string().replace("\"","").chars().collect::<Vec<char>>();
            if chr_vec.len()==0{
                continue;
            }
            if val_vec.len()==0{
                continue;
            }
            let chr = chr_vec[0];
            let val = val_vec[0];
            let tok = types::deva_to_enum(val);
            match category.as_str(){
                "vowels" => map_to_tokens.Vowels.insert(chr,tok),
                "vowel_marks" => map_to_tokens.VowelMarks.insert(chr,tok),
                "accents" => map_to_tokens.Accents.insert(chr,tok),
                "consonants" => map_to_tokens.Consonants.insert(chr,tok),
                "yogavaahas" => map_to_tokens.Yogavāhas.insert(chr,tok),
                "symbols" => map_to_tokens.Symbols.insert(chr,tok),
                "virama" => map_to_tokens.Virāmam.insert(chr,tok),
                _ => None
            };
        }
    }
    map_to_tokens
}





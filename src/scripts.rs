use std::collections::{HashMap, HashSet};
use toml;

#[derive(Debug, Clone)]
pub enum ScriptName{
    Telugu,
    IastIso,
    Devanagari
}

type ScriptMap = HashMap<String, HashMap<String, toml::Value>>;

pub fn read_script(which: &ScriptName)-> ScriptMap{

    let script = match *which{
      ScriptName::Telugu => include_str!("../common_maps/brahmic/telugu.toml"),
      ScriptName::IastIso => include_str!("../common_maps/roman/iast_iso_m.toml"),
      ScriptName::Devanagari => include_str!("../common_maps/brahmic/devanagari.toml"),
    };
    let script: ScriptMap = toml::from_str(script).unwrap();
    script
}

pub fn get_category(input_script: &ScriptName, output_script: &ScriptName, category_to_get: Option<&str>) -> HashMap<String, String>{
    /*
     * Takes two ScriptMaps that are the to and from scripts. Takes a category if only one category
     * is desired. Else pass in None for all categories. 
     */
    let left = read_script(input_script);
    let right = read_script(output_script);
    
    let mut output_script: HashMap::<String, String> = HashMap::new();

    for category in left.keys(){
        if !category_to_get.is_none() && Some(category.clone().as_str()) != category_to_get{
            continue
        }
        let empty = HashMap::<String, toml::Value>::new();
        let left_category = left.get(category).cloned().unwrap_or(empty.clone());
        let right_category = right.get(category).cloned().unwrap_or(empty.clone());
        
        for character in left_category.keys(){
            let mut left_values: toml::Value = toml::Value::String("".to_string());
            let mut right_values: toml::Value = toml::Value::String("".to_string());
            
            if let Some(l) = left_category.get(character){
                left_values = l.clone();
            } 
            if let Some(r) = right_category.get(character){
                right_values = r.clone();
            };

            let left_val = match left_values{
             toml::Value::String(x) => Some(x),
             _ => None
            };

            let right_val = match right_values{
               toml::Value::String(x) => Some(x),
                _ => None
            };

            if !left_val.is_none() && !right_val.is_none(){
                output_script.insert(left_val.unwrap(), right_val.unwrap());
                continue;
            } else {
                // Note that left_val can't be none because it is alway a single char, never a
                // toml::Value::Array. 
                if right_val.is_none(){
                    println!("{}", format!("Support for 1:Many maps of characters not yet implemented. Skipping {character} for now."));
                }

            }
        }

    }
    output_script
}

pub fn get_category_by_lengths(input_script: &ScriptName, output_script: &ScriptName, category_to_get: Option<&str>) -> (Vec<HashMap<String, String>>, Vec<usize>){
    let mut category = get_category(input_script, output_script, category_to_get);
    let mut results = vec![];
    let mut result = HashMap::<String, String>::new();
    let mut lengths = HashSet::<usize>::new();    
    for (key, val) in category.iter(){
        lengths.insert(key.len());
    }
    let mut lengths: Vec<usize> = lengths.into_iter().collect();
    lengths.sort_by(|a, b| b.cmp(a));

    for l in lengths.iter(){
        for (key, val) in category.iter(){
            if key.len() == *l{
                    result.insert(key.to_string(), val.to_string());
            }
        }
        results.push(result.clone());
        result = HashMap::<String, String>::new();
    }
    println!("{:?}", results);
    (results, lengths)
}   


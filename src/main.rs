use std::fs;
use clap::{Arg, Command};
use std::env;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

mod scripts;
mod tests;

#[derive(Debug)]
pub struct UserInputContext{
    filename: String,
    input: Option<scripts::ScriptName>,
    output: Option<scripts::ScriptName>,
    input_char_lengths: Vec<usize>
}

#[derive(Debug)]
pub struct Text{
    original: Vec<char>,
    transliterated: String
}

pub fn generate_scriptname_from_arg(user_input: &str) -> Option<scripts::ScriptName> {
    match user_input{
        "devanagari" => Some(scripts::ScriptName::Devanagari),
        "telugu" => Some(scripts::ScriptName::Telugu),
        "iast_iso" => Some(scripts::ScriptName::IastIso),
        _ => None
    }
}

pub fn read_file(filename: &str) -> Text {
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(code) => code,
        Err(e) => e.to_string(),
    };
    let mut contents_vector: Vec<char> = Vec::new();
    let mut raw_contents: Vec<&str> = UnicodeSegmentation::graphemes(contents.as_str(), true).collect::<Vec<&str>>();
   
    
    for strs in raw_contents.iter(){
        for chr in strs.chars(){
           contents_vector.push(chr); 
        }
    }
    
    Text{original: contents_vector, transliterated: "".to_string()}
}

fn create_context()->UserInputContext{
    let args = Command::new("transliterator")
        .arg(Arg::new("filename").required(true))
        .arg(Arg::new("input_script").required(true))
        .arg(Arg::new("output_script").required(true))
        .get_matches();
    let input_script: String = args.get_one::<String>("input_script").expect("input script required").clone();
    let output_script: String = args.get_one::<String>("output_script").expect("output script required").clone();
    let filename: String = args.get_one::<String>("filename").expect("output script required").clone();
    let mut context = UserInputContext{filename: filename, 
        input: None, 
        output: None, 
        input_char_lengths: vec![]};
    context.input = generate_scriptname_from_arg(&input_script);
    context.output = generate_scriptname_from_arg(&output_script);
    let filename: String = args.get_one::<String>("filename").expect("filename required").clone().to_string();
    context
}

fn generate_syllables(i: &usize, text: &Vec<char>) -> Vec<Option<String>>{
    let mut one = String::new();
    let mut two = String::new();
    let mut three = String::new();

    let a: char = *text.get(*i).unwrap();
    let mut b: char = '\0';
    let mut c: char = '\0';

    one.push(a.clone());
    let ret_one: Option<String> = Some(one);
    let mut ret_two: Option<String> = None;
    let mut ret_three: Option<String> = None;

    if *i<=text.len()-2{ 
        b = *text.get(*i+1).unwrap();
        two.push(a.clone());
        two.push(b.clone());
        ret_two = Some(two);
    }
    if *i<=text.len()-3{
        c = *text.get(*i+2).unwrap();
        three.push(a.clone());
        three.push(b.clone());
        three.push(c.clone());
        ret_three = Some(three);
    }
    println!("{} {:?} {:?} {:?}", *i, ret_three, ret_two, ret_one);
    vec![ret_three, ret_two, ret_one]
}

pub fn transliterate_token(text: &Vec<char>, start: usize, end: usize, mappings: &Vec<HashMap<String, String>>, lengths: &Vec<usize>) -> String{
    let mut original = String::new();
    let mut result = String::new();

    let mut skip_till = 0;
    for i in start..end{
        original.push('|');
        original.push(text[i].clone());
        if skip_till>i{
            continue;
        }
        
        let toks = generate_syllables(&i, text);
        let mut get: Option<&String>;
            
        'outer: for map in mappings{
            let mut skip_val = 3;
            for tok in 0..toks.len(){
                if !toks[tok].is_none(){
                    let search_token = toks[tok].as_ref().unwrap();
                    get = map.get(search_token);
                    println!("{search_token} -> {:?}", get);
                    if !get.is_none(){
                        let gotten = get.unwrap();
                        result.push_str(gotten);
                        skip_till = i+skip_val;
                        break 'outer;
                    }
                    skip_val -= 1;
            }
            }
        }

                    
    }

    println!("{original} -> {result}");
    result
}

pub fn tokens2transliterate(text: &mut Text, context: &UserInputContext){
    let all = scripts::get_category(context.input.as_ref().unwrap(), context.output.as_ref().unwrap(), None);
     let script_and_lengths = scripts::get_category_by_lengths(context.input.as_ref().unwrap(), context.output.as_ref().unwrap(), None);
     let mappings = script_and_lengths.0;
     let lengths = script_and_lengths.1;

        
    transliterate_token(&text.original, 0, 10, &mappings, &lengths);
}

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    let context = create_context();
    let mut text = read_file(context.filename.as_str());
    tokens2transliterate(&mut text, &context);
}

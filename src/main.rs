/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/

use lexer::{deva_to_enum, enum_to_deva, read_script};
use scanner::{indic_to_chars, roman_to_graphemes};
use std::fs;
use std::io::prelude::*;
use types::Token;

use crate::lexer::map_vowel_marks;

mod lexer;
mod scanner;
//todo!("Bring the types into the outer scope. It's annoying to type \'types\'.");
mod types;

#[derive(Debug)]
pub struct TransliterationContext {
    input: Option<types::ScriptName>,
    output: Option<types::ScriptName>,
    text: String,
    tokenized: Option<Vec<types::Token>>,
    transliterated: String,
}

fn generate_scriptname(value: String) -> Option<types::ScriptName> {
    match value.as_str() {
        "devanagari" => Some(types::ScriptName::Devanagari),
        "telugu" => Some(types::ScriptName::Telugu),
        "iast_iso" => Some(types::ScriptName::IastIso),
        _ => None,
    }
}

fn create_context(text: String, input: String, output: String) -> TransliterationContext {
    let mut context = TransliterationContext {
        text: text,
        input: generate_scriptname(input),
        output: generate_scriptname(output),
        tokenized: None,
        transliterated: "".to_string(),
    };
    context
}

pub fn text_to_tokens(ctx: &mut TransliterationContext) {
    let text_vector: Vec<String>;
    if ctx.input.unwrap() < types::ScriptName::IastIso {
        text_vector = indic_to_chars(ctx.text.clone());
    } else {
        text_vector = roman_to_graphemes(ctx.text.clone());
    }
    let map_to_deva = lexer::produce_scriptmap(&mut lexer::read_script(&ctx.input.unwrap()), true);
    let mut tokenized: Vec<types::Token> = vec![];
    for i in text_vector.iter() {
        for group_name in lexer::CHAR_GROUPS {
            if map_to_deva.get(group_name).unwrap().contains_key(i) {
                let replacement = map_to_deva.get(group_name).unwrap().get(i).unwrap();
                let token = deva_to_enum(replacement.to_string());
                if tokenized.last() != None {
                    if token.is_vowel() && tokenized.last().unwrap().is_consonant() {
                        tokenized.push(Token::virama);
                    }
                }
                tokenized.push(token);
            }
        }
    }
    ctx.tokenized = Some(tokenized);
}

pub fn tokens_to_text(ctx: &mut TransliterationContext) {
    let roman = ctx.output.unwrap().is_roman();

    let map_to_output =
        lexer::produce_scriptmap(&mut lexer::read_script(&ctx.output.unwrap()), false);
    let mut prev: Token = Token::virama;
    let mut start = true;
    let tokens = ctx.tokenized.clone().unwrap();
    println!("{:?}", map_to_output);
    for tok in tokens.iter() {
        let token = *tok;
        let mut c = enum_to_deva(token);
        let mut group_name = "vowels";
        if token.is_vowel() && !start {
            group_name = "vowel_marks";
        } else if token.is_consonant() {
            group_name = "consonants";
        } else if token.is_virama() {
            group_name = "virama";
        } else if token.is_accent() {
            group_name = "accents";
        } else if token == Token::whitespace || token == Token::newline {
            ctx.transliterated.push_str(&c);
            continue;
        }

        if group_name == "vowel_marks" {
            c = map_vowel_marks(token);
        }

        let group = match map_to_output.get(group_name) {
            Some(x) => x.get(&c),
            None => None,
        };
        let push = match group {
            Some(x) => x,
            None => &c,
        };
        println!("{} {:?} {} {}", group_name, token, c, push);

        if !start && roman && prev.is_consonant() && !token.is_virama() {
            ctx.transliterated
                .push_str(map_to_output.get("vowels").unwrap().get("अ").unwrap());
        }

        if token.is_vowel() && !start {
            ctx.transliterated.pop();
        }
        ctx.transliterated.push_str(push);
        prev = token.clone();
        start = false;
    }
}

fn main() {
    let txt = "इ॒षे त्वो॒र्जे".to_string();
    let txt = "अश्म॒न्नूर्जं॒ पर्व॑ते शिश्रिया॒णां वाते॑ प॒र्जन्ये॒ वरु॑णस्य॒ शुष्मे᳚ ।
       अ॒द्भ्य ओष॑धीभ्यो॒ वन॒स्पति॒भ्योऽधि॒ संभृ॑तां॒ तां न॒ इष॒मूर्जं॑
       धत्त मरुतः सꣳ ररा॒णाः ॥ अश्मग्ग्॑स्ते॒ क्षुद॒मुं ते॒ शुगृ॑च्छतु॒
       यं द्वि॒ष्मः ॥ स॒मु॒द्रस्य॑ त्वा॒ऽवाक॒याग्ने॒ परि॑ व्ययामसि । पा॒व॒को
       अ॒स्मभ्यꣳ॑ शि॒वो भ॑व ॥ हि॒मस्य॑ त्वा ज॒रायु॒णाग्ने॒ परि॑ व्ययामसि ।
       पा॒व॒को अ॒स्मभ्यꣳ॑ शि॒वो भ॑व ॥"
        .to_string();

    let mut ctx = create_context(txt, "devanagari".to_string(), "iast_iso".to_string());
    text_to_tokens(&mut ctx);
    tokens_to_text(&mut ctx);

    println!("TOKENS {:?}", ctx.tokenized);
    println!("RESULT {}", ctx.transliterated);
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let text = "empty".to_string();
        let ctx = create_context(text.clone(), "telugu".to_string(), "devanagari".to_string());

        assert_eq!(ctx.input, Some(types::ScriptName::Telugu));
        assert_eq!(ctx.output, Some(types::ScriptName::Devanagari));
        assert_eq!(ctx.text, text);
    }

    #[test]
    fn test_tokenizer() {
        let text = "इ॒षे त्वो॒र्जे".to_string();
        let mut ctx = create_context(
            text.clone(),
            "devanagari".to_string(),
            "iast_iso".to_string(),
        );

        text_to_tokens(&mut ctx);
        assert_eq!(ctx.input, Some(types::ScriptName::Devanagari));
        assert_eq!(ctx.output, Some(types::ScriptName::IastIso));
        assert_eq!(ctx.text, text);
    }
}

/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/

use lexer::{deva_to_enum, read_script};
use scanner::{indic_to_chars, roman_to_graphemes};

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
    };
    context
}

pub fn text_to_tokens(ctx: &mut TransliterationContext) {
    let mut text_vector: Vec<String>;
    if ctx.input.unwrap() < types::ScriptName::IastIso {
        text_vector = indic_to_chars(ctx.text.clone());
    } else {
        text_vector = roman_to_graphemes(ctx.text.clone());
    }

    let map_to_deva = lexer::produce_scriptmap(&mut lexer::read_script(&ctx.input.unwrap()), true);
    let mut tokenized: Vec<types::Token> = vec![];
    println!("{}", ctx.text);
    println!("{text_vector:?}");
    for i in text_vector.iter() {
        for group_name in lexer::CHAR_GROUPS {
            if map_to_deva.get(group_name).unwrap().contains_key(i) {
                let replacement = map_to_deva.get(group_name).unwrap().get(i).unwrap();
                let token = deva_to_enum(replacement.to_string());
                if (token.is_viraama() && tokenized.last().unwrap().is_vowel())
                    || (token.is_vowel() && tokenized.last().unwrap().is_vowel())
                {
                    tokenized.pop();
                    tokenized.push(token);
                    break;
                }
                if token.is_consonant() {
                    tokenized.push(token);
                    tokenized.push(types::Token::a);
                    break;
                }
                tokenized.push(token);
                break;
            }
        }
    }
    println!("{tokenized:?}");
}
fn main() {
    let mut ctx = create_context(
        "कैलासशिखरासीनम् अशेषामरपूजितम्
कालघ्नं श्रीमहाकालम् ईश्वरं ज्ञानपारगम् ॥
संपूज्य विधिवद् भक्त्या ऋष्यात्रेयः सुसंयतः
सर्वभूतहितार्थाय पप्रच्छेदं महामुनिः ॥
ज्ञानयोगं न विन्दन्ति ये नरा मन्दबुद्धयः
ते मुच्यन्ते कथं घोराद् भगवन् भवसागरात् ॥
ए"
        .to_string(),
        "devanagari".to_string(),
        "telugu".to_string(),
    );
    text_to_tokens(&mut ctx);
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn text_context_creation() {
        let text = "empty".to_string();
        let ctx = create_context(text.clone(), "telugu".to_string(), "devanagari".to_string());

        assert_eq!(ctx.input, Some(types::ScriptName::Telugu));
        assert_eq!(ctx.output, Some(types::ScriptName::Devanagari));
        assert_eq!(ctx.text, text);
    }
}

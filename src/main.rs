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
pub struct Context {
    input: types::ScriptName,
    output: types::ScriptName,
}

impl Context {
    fn new(input: String, output: String) -> Self {
        Self {
            input: generate_scriptname(input),
            output: generate_scriptname(output),
        }
    }
}

fn generate_scriptname(value: String) -> types::ScriptName {
    match value.as_str() {
        "devanagari" => types::ScriptName::Devanagari,
        "telugu" => types::ScriptName::Telugu,
        "iast_iso" => types::ScriptName::IastIso,
        _ => panic!("Unknown value: {value}"),
    }
}

fn create_context(text: String, input: String, output: String) -> Context {
    Context::new(input, output)
}

fn text_to_tokens(text: &str, ctx: Context) -> Vec<Token> {
    for c in text.chars(){
        
    }
    vec![]
}

fn tokens_to_output(ctx: Context, tokens: &Vec<Token>) {

}

fn main() {
    let text = "अश्म॒न्नूर्जं॒ पर्व॑ते शिश्रिया॒णां वाते॑ प॒र्जन्ये॒ वरु॑णस्य॒ शुष्मे᳚ ।
       अ॒द्भ्य ओष॑धीभ्यो॒ वन॒स्पति॒भ्योऽधि॒ संभृ॑तां॒ तां न॒ इष॒मूर्जं॑
       धत्त मरुतः सꣳ ररा॒णाः ॥ अश्मग्ग्॑स्ते॒ क्षुद॒मुं ते॒ शुगृ॑च्छतु॒
       यं द्वि॒ष्मः ॥ स॒मु॒द्रस्य॑ त्वा॒ऽवाक॒याग्ने॒ परि॑ व्ययामसि । पा॒व॒को
       अ॒स्मभ्यꣳ॑ शि॒वो भ॑व ॥ हि॒मस्य॑ त्वा ज॒रायु॒णाग्ने॒ परि॑ व्ययामसि ।
       पा॒व॒को अ॒स्मभ्यꣳ॑ शि॒वो भ॑व ॥"
        .to_string();

    let mut ctx = Context::new("devanagari".to_string(), "iast_iso".to_string());
    let mut tokens: Vec<types::Token>  = vec![];
    //text_to_tokens(&mut ctx, &mut tokens);
    //tokens_to_text(&mut ctx);

}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let ctx = Context::new("telugu".to_string(), "devanagari".to_string());
        assert_eq!(ctx.input, types::ScriptName::Telugu);
        assert_eq!(ctx.output, types::ScriptName::Devanagari);
    }

    #[test]
    fn test_tokenizer() {
        let text = "इ॒षे त्वो॒र्जे".to_string();
        let mut ctx = Context::new(
            "devanagari".to_string(),
            "iast_iso".to_string(),
        );

        assert_eq!(ctx.input, types::ScriptName::Devanagari);
        assert_eq!(ctx.output, types::ScriptName::IastIso);
    }
}

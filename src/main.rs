/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/

mod lexer;
mod scanner;
//todo!("Bring the types into the outer scope. It's annoying to type \'types\'.");
mod types;

#[derive(Debug)]
struct TransliterationContext {
    input: Option<types::ScriptName>,
    output: Option<types::ScriptName>,
    text: String,
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
    };
    context
}

pub fn text_to_tokens(text: String) -> Vec<types::Token> {
    vec![]
}
fn main() {}

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

/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/

mod lexer;
mod scanner;
mod types;

use lexer::{deva_to_enum, produce_scriptmap, read_script};
use types::Script;
use types::Token;

#[derive(Debug)]
pub struct Context {
    input: Script,
    output: Script,
}

pub struct Transliterator {
    context_a: bool,
    context_b: bool,
}

impl Transliterator {
    pub fn new(context_a: bool, context_b: bool) -> Self {
        Self {
            context_a,
            context_b,
        }
    }
    pub fn transliterate(&self, text: &str, input: Script, output: Script) -> String {
        "".to_string()
    }
}

impl Context {
    fn new(input: String, output: String) -> Self {
        Self {
            input: generate_scriptname(input),
            output: generate_scriptname(output),
        }
    }
}

fn generate_scriptname(value: String) -> Script {
    match value.parse() {
        Ok(script) => script,
        _ => panic!("Unknown value: {value}"),
    }
}

fn tokenize(text: &str, ctx: &Context) -> Vec<Token> {
    let mut raw_mapping = read_script(ctx.input);
    let mapping = produce_scriptmap(&mut raw_mapping, true);
    let mut tokens: Vec<Token> = vec![];
    println!("INPUT TO TOK {:?}", mapping);
    for c in text.chars() {
        for char_category in mapping.values() {
            let char_key = c.to_string();
            if let Some(value) = char_category.get(&deva_to_enum(&char_key)) {
                let t = deva_to_enum(&value);

                if t.is_vowel_mark() | t.is_virama() {
                    tokens.pop();
                    tokens.push(t);
                } else if t.is_consonant() {
                    tokens.push(t);
                    tokens.push(Token::vm_a)
                } else {
                    tokens.push(t);
                }

                break;
            }
        }
    }

    tokens
}

fn render(tokens: &Vec<Token>, ctx: &Context) -> String {
    let mut output = String::new();
    let raw_mapping = read_script(ctx.output);
    let mapping = produce_scriptmap(&raw_mapping, false);
    println!("TOK TO OUTPUT {:?}", mapping);
    println!("TOKENS TO RENDER {:?}", tokens);
    for token in tokens.iter() {
        let char_key = token.to_devanagari();
        println!("FETCH FROM MAP {token:?} -> {char_key}");
        for char_category in mapping.values() {
            if let Some(value) = char_category.get(token) {
                println!("{:?} -> {} -> {}", token, char_key, value);
                output += value;
                break;
            }
        }
    }
    output
}

fn main() {
    let text = "अश्म॒न्नूर्जं॒ पर्व॑ते ".to_string();

    let ctx = Context::new("devanagari".to_string(), "iast_iso".to_string());
    let tokens = tokenize(&text, &ctx);
    println!("TOKENS {:?}", tokens);
    let output = render(&tokens, &ctx);
    println!("OUTPUT {}", output);
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let ctx = Context::new("telugu".to_string(), "devanagari".to_string());
        assert_eq!(ctx.input, Script::Telugu);
        assert_eq!(ctx.output, Script::Devanagari);
    }

    #[test]
    fn test_tokenizer_deva() {
        use Token::*;
        let text = "इ॒षे त्वो॒र्जे";
        let ctx = Context::new("devanagari".to_string(), "iast_iso".to_string());
        let tokens = tokenize(&text, &ctx);
        assert_eq!(
            tokens,
            vec![
                i,
                anudaatta,
                Sh,
                vm_E,
                Unknown(" ".to_string()),
                t,
                virama,
                v,
                vm_O,
                anudaatta,
                r,
                virama,
                j,
                vm_E
            ]
        )
    }

    fn test_tokenizer_iast_iso() {
        use Token::*;
        let text = "i̱ṣe tvo̱rje";
        let ctx = Context::new("iast_iso".to_string(), "devanagari".to_string());
        let tokens = tokenize(&text, &ctx);
        assert_eq!(
            tokens,
            vec![
                i,
                anudaatta,
                Sh,
                vm_E,
                Unknown(" ".to_string()),
                t,
                virama,
                v,
                vm_O,
                anudaatta,
                r,
                virama,
                j,
                vm_E
            ]
        )
    }

    fn test_render_iast_iso() {
        use Token::*;
        let text = "i̱ṣe tvo̱rje";
        let ctx = Context::new("iast_iso".to_string(), "devanagari".to_string());
        let tokens = tokenize(&text, &ctx);
        let result = render(&tokens, &ctx);
        assert_eq!(result, "इ॒षे त्वो॒र्जे")
    }
}

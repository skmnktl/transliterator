use regex::Regex;



fn match_tokens(c: &str) {

}

fn fix_separators_for_multichar_tokens(code: String) -> String {
    let multi_chars = vec![
        "<<=", ">>=", "->", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "+=", "-=", "*=", "/=",
        "%=", "&=", "^=", "|=", "--", "++",
    ];
    let mut code = code;
    for tok in multi_chars {
        let replacement = tok.clone().to_string();
        let replacement_move = replacement.clone();
        let search_for = add_separators_to_string(replacement_move);
        let replace_with = format!("`s`{replacement}`s`");
        //let re = Regex::new(&search_for).unwrap();
        // *code = re.replace_all(code, replace_with.as_str()).into_owned().to_string();
        code = code.replace(search_for.as_str(), replacement.as_str());
    }
    code
}

pub fn tokenize(contents: String) -> Vec<Token> {
    let separated = add_separators_to_string(contents);
    let separated = fix_separators_for_multichar_tokens(separated);
    let separated = separated.split("`s`");

    let mut tokens: Vec<Token> = Vec::new();
    for (i, t) in separated.enumerate() {
        if t.len() == 0 {
            continue;
        }

        let token = match_tokens(t);

        if token == Token::Name("".to_string()) {
            continue;
        }
        tokens.push(token);
    }

    mark_parens(&mut tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::scan;
    use crate::types::Token::*;
    use crate::types::Token;
    use crate::utils;
    
    

    #[test]
    fn test_tokenize_one() {
        let filename = "../sample_code/01_mystery.c0";
        let trg: Vec<Token> = vec![
            Int,
            Name("POW".to_string()),
            LParen(8),
            Int,
            Name("b".to_string()),
            Comma,
            Int,
            Name("e".to_string()),
            Minus,
            IntBox(1),
            RParen(2),
            LBrace(31),
            If,
            LParen(15),
            Name("e".to_string()),
            Eq,
            IntBox(0),
            RParen(11),
            Return,
            IntBox(1),
            Semicolon,
            Return,
            Name("POW".to_string()),
            LParen(27),
            Name("b".to_string()),
            Comma,
            Name("e".to_string()),
            Minus,
            IntBox(1),
            RParen(21),
            Star,
            Name("b".to_string()),
            Semicolon,
            RBrace(9),
            Int,
            Name("f".to_string()),
            LParen(40),
            Int,
            Name("x".to_string()),
            Comma,
            Int,
            Name("y".to_string()),
            RParen(34),
            Requires,
            Name("y".to_string()),
            GEq,
            IntBox(0),
            Semicolon,
            Ensures,
            Name("POW".to_string()),
            LParen(52),
            Name("x".to_string()),
            Comma,
            Name("y".to_string()),
            RParen(48),
            Eq,
            ResultCondition,
            Semicolon,
            LBrace(133),
            Int,
            Name("b".to_string()),
            Assign,
            Name("x".to_string()),
            Semicolon,
            Int,
            Name("e".to_string()),
            Assign,
            Name("y".to_string()),
            Semicolon,
            Int,
            Name("r".to_string()),
            Assign,
            IntBox(1),
            Semicolon,
            While,
            LParen(77),
            Name("e".to_string()),
            GThan,
            IntBox(0),
            RParen(73),
            LoopInvariant,
            Name("e".to_string()),
            GEq,
            IntBox(0),
            Semicolon,
            LoopInvariant,
            Name("POW".to_string()),
            LParen(89),
            Name("b".to_string()),
            Comma,
            Name("e".to_string()),
            RParen(85),
            Star,
            Name("r".to_string()),
            Eq,
            Name("POW".to_string()),
            LParen(98),
            Name("x".to_string()),
            Comma,
            Name("y".to_string()),
            RParen(94),
            Semicolon,
            LBrace(129),
            If,
            LParen(108),
            Name("e".to_string()),
            Mod,
            IntBox(2),
            Eq,
            IntBox(1),
            RParen(102),
            LBrace(116),
            Name("r".to_string()),
            Assign,
            Name("b".to_string()),
            Star,
            Name("r".to_string()),
            Semicolon,
            RBrace(109),
            Name("b".to_string()),
            Assign,
            Name("b".to_string()),
            Star,
            Name("b".to_string()),
            Semicolon,
            Name("e".to_string()),
            Assign,
            Name("e".to_string()),
            Div,
            IntBox(2),
            Semicolon,
            RBrace(100),
            Return,
            Name("r".to_string()),
            Semicolon,
            RBrace(56),
        ];

        let contents = utils::read_file(filename);
        let no_comments = scan::strip_comments(contents);
        let mut src = lex::tokenize(no_comments);
        assert_eq!(src, trg);
    }
}

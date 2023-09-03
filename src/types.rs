use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(C)]
#[allow(unused_assignments)]
pub enum Token {
    // Vowels
    a = 1,
    A,
    i,
    I,
    u,
    U,
    RRi,
    RRI,
    LLi,
    LLI,
    _e,
    e,
    E,
    ai,
    _o,
    o,
    O,
    R, // tamil ra or bandira in telugu
    au,
    //Yogavahas
    M = 100,
    H,
    cbindu__y,
    // Symbols
    visarga = 200,
    cbindu,
    danda,
    dvidanda,
    // Svara
    anudaatta = 300,
    udaatta,
    svarita,
    gm,
    gg,
    //virama
    viraama = 400,
    // Consonants
    k = 500,
    kh,
    g,
    gh,
    ng,
    c,
    ch,
    j,
    jh,
    ny,
    T,
    Th,
    D,
    Dh,
    N,
    t,
    th,
    d,
    dh,
    n,
    p,
    ph,
    b,
    bh,
    m,
    y,
    r,
    l,
    v,
    sh,
    Sh,
    s,
    h,
    L,
    //Others
    unk = 600,
    whitespace,
    newline,
}

impl Token {
    fn is_vowel(&self) -> bool {
        if *self < Token::M {
            return true;
        }
        return false;
    }

    fn is_yogavaha(&self) -> bool {
        if (*self < Token::visarga) && (*self >= Token::M) {
            return true;
        }
        return false;
    }

    fn is_consonant(&self) -> bool {
        if (*self < Token::unk) && (*self >= Token::k) {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptName {
    Devanagari,
    Telugu,
    IastIso,
}

pub type RawScriptMap = HashMap<String, HashMap<String, toml::Value>>;
pub type ScriptMap = HashMap<String, HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel() {
        assert_eq!(Token::a.is_vowel(), true);
    }
    #[test]
    fn test_not_vowel() {
        assert_eq!(Token::M.is_vowel(), false);
    }

    #[test]
    fn test_yogavaha() {
        assert_eq!(Token::M.is_yogavaha(), true);
    }

    #[test]
    fn test_not_yogavaha() {
        assert_eq!(Token::a.is_yogavaha(), false);
    }

    #[test]
    fn test_consonant() {
        assert_eq!(Token::k.is_consonant(), true);
    }

    #[test]
    fn test_not_consonant() {
        assert_eq!(Token::a.is_yogavaha(), false);
    }
}

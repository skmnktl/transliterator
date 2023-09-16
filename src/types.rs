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
    virama = 400,
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
    pub fn is_vowel(&self) -> bool {
        if *self < Token::M {
            return true;
        }
        return false;
    }

    pub fn is_yogavaha(&self) -> bool {
        if (*self < Token::visarga) && (*self >= Token::M) {
            return true;
        }
        return false;
    }

    pub fn is_consonant(&self) -> bool {
        if (*self < Token::unk) && (*self >= Token::k) {
            return true;
        }
        return false;
    }

    pub fn is_virama(&self) -> bool {
        if (*self == Token::virama) {
            return true;
        }
        return false;
    }

    pub fn is_accent(&self) -> bool {
        if (*self >= Token::anudaatta && *self <= Token::gg) {
            return true;
        }
        return false;
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(C)]
pub enum ScriptName {
    // Brahmic scripts are enumerated starting at 1.
    Devanagari = 1,
    Telugu,
    // All roman scripts are in the 100 group
    IastIso = 100,
}

impl ScriptName {
    pub fn is_roman(&self) -> bool {
        if *self >= ScriptName::IastIso {
            return true;
        }
        false
    }
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

    #[test]
    fn test_viraama() {
        assert_eq!(Token::viraama.is_viraama(), true);
    }

    #[test]
    fn test_not_viraama() {
        assert_eq!(Token::a.is_viraama(), false);
    }

    #[test]
    fn test_accent() {
        assert_eq!(Token::udaatta.is_viraama(), true);
    }

    #[test]
    fn test_not_accent() {
        assert_eq!(Token::a.is_accent(), false);
    }
}

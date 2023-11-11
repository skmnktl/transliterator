use std::collections::HashMap;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Debug, Clone)]
#[allow(unused_assignments)]
pub enum Token {
    // Vowels
    a,
    A,
    i,
    I,
    u,
    U,
    RRi,
    RRI,
    LLi,
    LLI,
    e,
    E,
    ai,
    o,
    O,
    R, // tamil ra or bandira in telugu
    au,
    // Vowel marks
    vm_a,
    vm_A,
    vm_i,
    vm_I,
    vm_u,
    vm_U,
    vm_RRi,
    vm_RRI,
    vm_LLi,
    vm_LLI,
    vm_e,
    vm_E,
    vm_ai,
    vm_o,
    vm_O,
    vm_R,
    vm_au,
    // Yogavahas
    M,
    H,
    cbindu__y,
    // Symbols
    visarga,
    cbindu,
    danda,
    dvidanda,
    // Svaras
    anudaatta,
    udaatta,
    svarita,
    gm,
    gg,
    // Virama
    virama,
    // Consonants
    k,
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
    // Others
    Unknown(String),
}

impl Token {
    pub fn is_vowel(&self) -> bool {
        use Token::*;
        matches!(
            self,
            a | A | i | I | u | U | RRi | RRI | LLi | LLI | e | E | ai | o | O | O | R | au
        )
    }

    pub fn is_vowel_mark(&self) -> bool {
        use Token::*;
        matches!(
            self,
            vm_a | vm_A
                | vm_i
                | vm_I
                | vm_u
                | vm_U
                | vm_RRi
                | vm_RRI
                | vm_LLi
                | vm_LLI
                | vm_e
                | vm_E
                | vm_ai
                | vm_o
                | vm_O
                | vm_R
                | vm_au
        )
    }

    pub fn is_yogavaha(&self) -> bool {
        use Token::*;
        matches!(self, M | H | cbindu__y)
    }

    pub fn is_consonant(&self) -> bool {
        use Token::*;
        matches!(
            self,
            k | kh
                | g
                | gh
                | ng
                | c
                | ch
                | j
                | jh
                | ny
                | T
                | Th
                | D
                | Dh
                | N
                | t
                | th
                | d
                | dh
                | n
                | p
                | ph
                | b
                | bh
                | m
                | y
                | r
                | l
                | v
                | sh
                | Sh
                | s
                | h
                | L
        )
    }

    pub fn is_virama(&self) -> bool {
        matches!(self, Token::virama)
    }

    pub fn is_accent(&self) -> bool {
        use Token::*;
        matches!(self, anudaatta | udaatta | svarita | gm | gg)
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Script {
    Devanagari,
    Telugu,
    IastIso,
}

impl Script {
    pub fn is_brahmic(&self) -> bool {
        !self.is_roman()
    }

    pub fn is_roman(&self) -> bool {
        use Script::*;
        matches!(self, IastIso)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError;

impl FromStr for Script {
    type Err = ConfigError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let ret = match value {
            "devanagari" => Self::Devanagari,
            "telugu" => Self::Telugu,
            "iast_iso" => Self::IastIso,
            _ => return Err(ConfigError),
        };
        Ok(ret)
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
    fn test_virama() {
        assert_eq!(Token::virama.is_virama(), true);
    }

    #[test]
    fn test_not_virama() {
        assert_eq!(Token::a.is_virama(), false);
    }

    #[test]
    fn test_accent() {
        assert_eq!(Token::udaatta.is_accent(), true);
    }

    #[test]
    fn test_not_accent() {
        assert_eq!(Token::a.is_accent(), false);
    }
}

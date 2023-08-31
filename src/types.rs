#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
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
    N__y,
    // Symbols
    visarga = 200,
    cbindu,
    // Svara
    anudaatta = 300,
    udaatta,
    svarita,
    // Consonants
    k = 400,
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
    sa,
    h,
    L,
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
}

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
}

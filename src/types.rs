
/* 
  What is a token? 
    A token is either a Vowel, VowelMark, Consonant, Yogavaha, Symbol, or Svara.
    To check if a given type is a Token, we need to implement an is_token method. 
    is_token always returns true when it is implemented. But having the trait lets use
    do some very nice type checking. 

    Later on, we can also consider implementing token addition and sandhi rules within the train.
*/
pub trait Token {
       
    fn is_token(&self) -> bool{
        true
    }

} 

#[allow(non_camel_case_types)]
pub enum Vowel {
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
    _e,
    e,
    E,
    ai,
    _o,
    o,
    O,
    R, // tamil ra or bandira in telugu
    au,
}

#[allow(non_camel_case_types)]
pub enum VowelMark {
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
    _e,
    e,
    E,
    ai,
    _o,
    o,
    O,
    R, // tamil ra or bandira in telugu
    au,
}


#[allow(non_camel_case_types)]
pub enum Yogavaha {
    M,
    H,
    N,
}


#[allow(non_camel_case_types)]
pub enum Symbol {
    h,
    cbindu,
}


#[allow(non_camel_case_types)]
pub enum Svara {
    anudaatta,
    udaatta,
    svarita,
}


#[allow(non_camel_case_types)]
pub enum Consonant {
    k,
    kh,
    g,
    gh,
    _N,
    c,
    ch,
    j,
    jh,
    _n,
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


impl Token for Vowel{}

impl Token for VowelMark{}

impl Token for Consonant{}

impl Token for Yogavaha{}

impl Token for Svara{}

impl Token for Symbol{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_impl() {
        // verify that the Token trait is implemented for each of the token types. 
        let x = Vowel::a;
        assert_eq!(x.is_token(), true);

        let x = VowelMark::a;
        assert_eq!(x.is_token(), true);

        let x = Consonant::k;
        assert_eq!(x.is_token(), true);

        let x = Yogavaha::H;
        assert_eq!(x.is_token(), true);

        let x = Symbol::h;
        assert_eq!(x.is_token(), true);

        let x = Svara::anudaatta;
        assert_eq!(x.is_token(), true);
    }
}

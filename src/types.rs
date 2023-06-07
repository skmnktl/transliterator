use std::collections::HashMap;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Vowels {
    a,
    ā,
    i,
    ī,
    u,
    ū,
    ṛ,
    ṝ,
    ḷ,
    ḹ,
    è,
    e,
    ai,
    ò,
    o,
    au,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelMarks {
    a,
    ā,
    i,
    ī,
    u,
    ū,
    ṛ,
    ṝ,
    ḷ,
    ḹ,
    è,
    e,
    ai,
    ò,
    o,
    au,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Yogavāhas {
    ṃ,
    ḥ,
    candrabindu,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Virāma {
    virāma,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Accents {
    udātta,
    anudātta,
    svarita,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Consonants {
    k,
    kh,
    g,
    gh,
    ṅ,
    c,
    ch,
    j,
    jh,
    ñ,
    ṭ,
    ṭh,
    ḍ,
    ḍh,
    ṇ,
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
    ś,
    ṣ,
    s,
    h,
    L, // retroflex L is different because the unicode is being annoying
    kṣ,
    jñ,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Symbols {
    zero,
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    oṃ,
    avagraha,
    daṇḍa,
    dvidaṇḍa,
    whitespace,
    newline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<Vowels, VowelMarks, Accents, Consonants, Yogavāhas, Symbols, Virāma> {
    Vowel(Vowels),
    VowelMark(VowelMarks),
    Accent(Accents),
    Consonant(Consonants),
    Yogavāha(Yogavāhas),
    Symbol(Symbols),
    Virāmam(Virāma),
    Unk(char),
}

pub type TokensAggregated =
    Token<Vowels, VowelMarks, Accents, Consonants, Yogavāhas, Symbols, Virāma>;

impl fmt::Display for TokensAggregated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn deva_to_enum(token: char) -> TokensAggregated {
    match token {
        'अ' => Token::Vowel(Vowels::a),
        'आ' => Token::Vowel(Vowels::ā),
        'इ' => Token::Vowel(Vowels::i),
        'ई' => Token::Vowel(Vowels::ī),
        'उ' => Token::Vowel(Vowels::u),
        'ऊ' => Token::Vowel(Vowels::ū),
        'ऋ' => Token::Vowel(Vowels::ṛ),
        'ॠ' => Token::Vowel(Vowels::ṝ),
        'ए' => Token::Vowel(Vowels::e),
        'ऎ' => Token::Vowel(Vowels::è),
        'ऐ' => Token::Vowel(Vowels::ai),
        'ऒ' => Token::Vowel(Vowels::ò),
        'ओ' => Token::Vowel(Vowels::o),
        'औ' => Token::Vowel(Vowels::au),
        'क' => Token::Consonant(Consonants::k),
        'ख' => Token::Consonant(Consonants::kh),
        'ग' => Token::Consonant(Consonants::g),
        'घ' => Token::Consonant(Consonants::gh),
        'ङ' => Token::Consonant(Consonants::ṅ),
        'च' => Token::Consonant(Consonants::c),
        'छ' => Token::Consonant(Consonants::ch),
        'ज' => Token::Consonant(Consonants::j),
        'झ' => Token::Consonant(Consonants::jh),
        'ञ' => Token::Consonant(Consonants::ñ),
        'ट' => Token::Consonant(Consonants::ṭ),
        'ठ' => Token::Consonant(Consonants::ṭh),
        'ड' => Token::Consonant(Consonants::ḍ),
        'ढ' => Token::Consonant(Consonants::ḍh),
        'ण' => Token::Consonant(Consonants::ṇ),
        'त' => Token::Consonant(Consonants::t),
        'थ' => Token::Consonant(Consonants::th),
        'द' => Token::Consonant(Consonants::d),
        'ध' => Token::Consonant(Consonants::dh),
        'न' => Token::Consonant(Consonants::n),
        'प' => Token::Consonant(Consonants::p),
        'फ' => Token::Consonant(Consonants::ph),
        'ब' => Token::Consonant(Consonants::b),
        'भ' => Token::Consonant(Consonants::bh),
        'म' => Token::Consonant(Consonants::m),
        'य' => Token::Consonant(Consonants::y),
        'र' => Token::Consonant(Consonants::r),
        'ल' => Token::Consonant(Consonants::l),
        'व' => Token::Consonant(Consonants::v),
        'श' => Token::Consonant(Consonants::ś),
        'ष' => Token::Consonant(Consonants::ṣ),
        'स' => Token::Consonant(Consonants::s),
        'ह' => Token::Consonant(Consonants::h),
        'ा' => Token::VowelMark(VowelMarks::ā),
        'ि' => Token::VowelMark(VowelMarks::i),
        'ी' => Token::VowelMark(VowelMarks::ī),
        'ु' => Token::VowelMark(VowelMarks::u),
        'ू' => Token::VowelMark(VowelMarks::ū),
        'ृ' => Token::VowelMark(VowelMarks::ṛ),
        'ॄ' => Token::VowelMark(VowelMarks::ṝ),
        'ॢ' => Token::VowelMark(VowelMarks::ḷ),
        'ॣ' => Token::VowelMark(VowelMarks::ḹ),
        'ॆ' => Token::VowelMark(VowelMarks::è),
        'े' => Token::VowelMark(VowelMarks::e),
        'ै' => Token::VowelMark(VowelMarks::ai),
        'ॊ' => Token::VowelMark(VowelMarks::ò),
        'ो' => Token::VowelMark(VowelMarks::o),
        'ौ' => Token::VowelMark(VowelMarks::au),
        'ँ' => Token::Yogavāha(Yogavāhas::candrabindu),
        'ं' => Token::Yogavāha(Yogavāhas::ṃ),
        'ः' => Token::Yogavāha(Yogavāhas::ḥ),
        '्' => Token::Virāmam(Virāma::virāma),
        '॑' => Token::Accent(Accents::udātta),
        '॒' => Token::Accent(Accents::anudātta),
        '᳡' => Token::Accent(Accents::svarita),
        '।' => Token::Symbol(Symbols::daṇḍa),
        '॥' => Token::Symbol(Symbols::dvidaṇḍa),
        ' ' => Token::Symbol(Symbols::whitespace),
        '\n' => Token::Symbol(Symbols::newline),
        _ => Token::Unk(token),
    }
}

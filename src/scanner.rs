pub fn indic_to_chars(input: String) -> Vec<String> {
    input.chars().map(|c| c.to_string()).collect::<_>()
}

pub fn roman_to_graphemes(input: String) -> Vec<String> {
    let result = input
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_graphemes() {
        let input: String = "mahāsarasvatī".to_string();
        let expected = vec![
            "m", "a", "h", "ā", "s", "a", "r", "a", "s", "v", "a", "t", "ī",
        ]
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
        assert_eq!(roman_to_graphemes(input), expected);
    }

    #[test]
    fn test_indic_to_chars() {
        let input: String = "महासरस्वती".to_string();
        let expected = vec!["म", "ह", "ा", "स", "र", "स", "\u{94d}", "व", "त", "ी"]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(indic_to_chars(input), expected);
    }

    #[test]
    fn test_indic_accented_to_chars() {
        let input: String = "इ॒षे त्वो॒र्जे त्वा॑ वा॒यवः॑ स्थोपा॒यव॑स्स्थ दे॒वो व॑स्सवि॒ता प्रार्प॑यतु॒".to_string();
        let expected = vec![
            "इ", "\u{952}", "ष", "\u{947}", " ", "त", "\u{94d}", "व", "ो", "\u{952}", "र",
            "\u{94d}", "ज", "\u{947}", " ", "त", "\u{94d}", "व", "ा", "\u{951}", " ", "व", "ा",
            "\u{952}", "य", "व", "ः", "\u{951}", " ", "स", "\u{94d}", "थ", "ो", "प", "ा",
            "\u{952}", "य", "व", "\u{951}", "स", "\u{94d}", "स", "\u{94d}", "थ", " ", "द",
            "\u{947}", "\u{952}", "व", "ो", " ", "व", "\u{951}", "स", "\u{94d}", "स", "व", "ि",
            "\u{952}", "त", "ा", " ", "प", "\u{94d}", "र", "ा", "र", "\u{94d}", "प", "\u{951}",
            "य", "त", "\u{941}", "\u{952}",
        ]
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
        assert_eq!(indic_to_chars(input), expected);
    }
}

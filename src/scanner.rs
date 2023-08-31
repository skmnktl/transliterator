/*
 * Objective: 
 *  For each input, we must generate a vector of chars for a given input. 
 *  There are two ways about this. One is to produce graphemes. That is, we can produce 
 *  what renders to the eye as a single character, but is in fact a set of characters that 
 *  have combined. Alternatively, we could parse all characters out into distinct units
 *  disregarding the fact that a character is meant to combine. 
 *
 *  We include functions to produce chars and graphemes for both indic and roman inputs   
 *  although we will only use one or the other for transliteration.
 */
use unicode_segmentation::UnicodeSegmentation;

pub fn indic_to_graphemes(input: String) -> Vec<String>{
    let val = input.as_str();
    let result  = UnicodeSegmentation::graphemes(input.as_str(), true).into_iter().collect::<Vec<&str>>();
    let result: Vec<String> = result.iter().map(|&s|s.into()).collect();
    result
    
}

pub fn indic_to_chars(input: String) -> Vec<String>{
    let result = input.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    result.to_vec()
}


pub fn roman_to_graphemes(input: String) -> Vec<String>{
    let result = input.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roman_to_graphemes() {
        let input: String = "mahāsarasvatī".to_string();
        let expected = vec!["m","a","h","ā","s","a","r","a","s","v","a","t","ī"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(roman_to_graphemes(input), expected);
   
    }
    
    #[test]
    fn test_indic_to_graphemes() {
        let input: String = "महासरस्वती".to_string();
        let expected = vec!["म", "हा", "स", "र", "स्", "व", "ती"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(indic_to_graphemes(input), expected);
   
    }

    #[test]
    fn test_indic_accented_to_graphemes() {
        let input: String = "इ॒षे त्वो॒र्जे त्वा॑ वा॒यवः॑ स्थोपा॒यव॑स्स्थ दे॒वो व॑स्सवि॒ता प्रार्प॑यतु॒".to_string();
        let expected = vec!["इ\u{952}", "ष\u{947}", " ", "त\u{94d}", "वो\u{952}", "र\u{94d}", "ज\u{947}", " ", "त\u{94d}", "वा\u{951}", " ", "वा\u{952}", "य", "वः\u{951}", " ", "स\u{94d}", "थो", "पा\u{952}", "य", "व\u{951}", "स\u{94d}", "स\u{94d}", "थ", " ", "द\u{947}\u{952}", "वो", " ", "व\u{951}", "स\u{94d}", "स", "वि\u{952}", "ता", " ", "प\u{94d}", "रा", "र\u{94d}", "प\u{951}", "य", "त\u{941}\u{952}"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(indic_to_graphemes(input), expected);
   
    }

    #[test]
    fn test_indic_to_chars() {
        let input: String = "महासरस्वती".to_string();
        let expected = vec!["म", "ह", "ा", "स", "र", "स", "\u{94d}", "व", "त", "ी"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(indic_to_chars(input), expected);
   
    }

    #[test]
    fn test_indic_accented_to_chars() {
        let input: String = "इ॒षे त्वो॒र्जे त्वा॑ वा॒यवः॑ स्थोपा॒यव॑स्स्थ दे॒वो व॑स्सवि॒ता प्रार्प॑यतु॒".to_string();
        let expected = vec!["इ", "\u{952}", "ष", "\u{947}", " ", "त", "\u{94d}", "व", "ो", "\u{952}", "र", "\u{94d}", "ज", "\u{947}", " ", "त", "\u{94d}", "व", "ा", "\u{951}", " ", "व", "ा", "\u{952}", "य", "व", "ः", "\u{951}", " ", "स", "\u{94d}", "थ", "ो", "प", "ा", "\u{952}", "य", "व", "\u{951}", "स", "\u{94d}", "स", "\u{94d}", "थ", " ", "द", "\u{947}", "\u{952}", "व", "ो", " ", "व", "\u{951}", "स", "\u{94d}", "स", "व", "ि", "\u{952}", "त", "ा", " ", "प", "\u{94d}", "र", "ा", "र", "\u{94d}", "प", "\u{951}", "य", "त", "\u{941}", "\u{952}"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(indic_to_chars(input), expected);
   
    }



}

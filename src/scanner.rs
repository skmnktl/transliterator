/*
 * Objective: For each input, we must generate a vector of chars. 
 *
 */
use unicode_segmentation::UnicodeSegmentation;

pub fn indic_to_chars(input: String) -> Vec<String>{
    let val = input.as_str();
    let result  = UnicodeSegmentation::graphemes(input.as_str(), true).into_iter().collect::<Vec<&str>>();
    let result: Vec<String> = result.iter().map(|&s|s.into()).collect();
    result
    
}


pub fn roman_to_chars(input: String) -> Vec<String>{
    let result = input.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roman_to_chars() {
        let input: String = "mahāsarasvatī".to_string();
        let expected = vec!["m","a","h","ā","s","a","r","a","s","v","a","t","ī"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(roman_to_chars(input), expected);
   
    }
    
    #[test]
    fn test_indic_to_chars() {
        let input: String = "महासरस्वती".to_string();
        let expected = vec!["म", "हा", "स", "र", "स्", "व", "ती"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(indic_to_chars(input), expected);
   
    }


}

/*
 * Objective: For each input, we must generate a vector of chars. 
 *
 */
use unicode_segmentation::UnicodeSegmentation;

pub fn indic_to_chars() -> Vec<char>{
    todo!()
}

pub fn roman_to_chars(input: &String) -> Vec<char>{
    let result = input.chars().collect();
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_chars() {
        let input: String = "mahāsarasvatī".to_string();
        let expected = vec!['m','a','h','ā','s','a','r','a','s','v','a','t','ī'];
        println!("{:?}", roman_to_chars(&input));
        assert_eq!(roman_to_chars(&input), expected);
   
    }
    
    #[test]
    fn test_indic_to_chars() {
        let input: String = "महासरस्वती".to_string();
        let expected = vec!['म', 'ह', 'ा', 'स','र','स', '्','व', 'त', 'ी'];
        println!("{:?}", roman_to_chars(&input));
        assert_eq!(roman_to_chars(&input), expected);
   
    }

}

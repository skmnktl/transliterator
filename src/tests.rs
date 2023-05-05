#[cfg(test)]
mod scripts_tests{

    use crate::scripts;
    #[test]
    fn test_read(){
        let read_lines = scripts::read_script(scripts::ScriptName::Devanagari);
        assert_eq!(read_lines.is_empty(), false);
    } 

    #[test]
    fn test_mapping_two_scripts(){
        let res = scripts.map_input_to_output(scripts::ScriptName::Telugu, scripts::ScriptName::IastIso);
        assert!(!res.is_empty())
    }
}

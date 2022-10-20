use toml::Value;
use toml::value::Table;
use toml::map::Map;

mod scripts;

fn load_script(lipi: &str) -> Table{
    let binding = match lipi{
        "TELUGU" => scripts::TELUGU.parse::<Value>().unwrap(),
            _ =>scripts::IAST_ISO.parse::<Value>().unwrap(),
     };
    // as_table returns a reference to a table.
    // Cloning returns a new table and passes ownership.
    let layered = binding.as_table().unwrap().clone();
    layered
}

fn get_vowels(lipi: &str) -> Table{
    let script = load_script(&lipi);
    let script = script["vowels"].as_table().unwrap().clone();
    script
}

fn construct_mapping(source: &str, target: &str){
    let src_lipi = load_script(source);
    let trg_lipi = load_script(target);
    
    for charset in src_lipi.keys(){  
        if trg_lipi.contains_key(charset) {
            let trg_tbl = trg_lipi[charset].as_table().unwrap();
        
            for (key, value) in src_lipi[charset].as_table().unwrap().iter(){

                if trg_tbl.contains_key(key){
                    let trg_sym = trg_tbl[key].as_str().unwrap();
                    println!("{value} => {trg_sym}");
                } else {
                    println!("Missing {value} character in target.")
                }
            }
        } else {
            println!("Missing {charset} chars in target.")
        }
    }
}

fn transliterate(input: &str) -> &str{
    let mut res: &str;
    let mut curr: &str;
    
    for c in input.chars(){
        unimplemented!()
    }

    "hello"
}

fn main() -> Result<(), ()> {
    let source_lipi_name = "IAST";
    let target_lipi_name = "TELUGU";
    
    construct_mapping(&source_lipi_name, &target_lipi_name);

    Ok(())
}





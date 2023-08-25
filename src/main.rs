/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/


use std::collections::HashMap;
use toml;

mod types;


#[derive(Debug, Clone)]
pub enum ScriptName {
    Devanagari,
}

type ScriptMap = HashMap<String, HashMap<String, toml::Value>>;

pub fn read_script(which: &ScriptName) -> ScriptMap {
    let script = match *which {
        ScriptName::Devanagari => include_str!("../maps/deva.toml"),
    };
    toml::from_str(script).unwrap()
}

fn main() {
    let script = ScriptName::Devanagari;
    println!("{:?}",read_script(&script));
}


mod test{
    //SLP1: aSman -> a, S, m, a, n
    //ITRANS: ashman -> a, sh, m, a, n

}
 

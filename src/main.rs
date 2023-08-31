/*
"sarasvatI" -> [Consonant::s,VowelMarks::a,r,a,s,v,a,t,I]

*/

mod lexer;
mod scanner;
mod types;

fn main() {
    let script = types::ScriptName::Devanagari;
    println!("{:?}", lexer::read_script(&script));
}

mod test {
    //SLP1: aSman -> a, S, m, a, n
    //ITRANS: ashman -> a, sh, m, a, n
}

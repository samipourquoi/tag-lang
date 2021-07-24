use crate::parser::AST;
use nom::error::Error;

mod parser;
mod generation;

fn main() {
    let result = parser::parse(r#"
        if true {
            if false {
                /say it's false!
            }
            /say hello world
        }
    "#).unwrap();
    dbg!(&result);
    generation::generate(result.1);
}

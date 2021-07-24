use crate::parser::AST;
use nom::error::Error;

mod parser;
mod generation;

fn main() {
    let result = parser::parse(r#"
        /say hello world
    "#).unwrap();
    // generation::generate(result.1);
    dbg!(result);
}

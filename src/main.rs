use crate::parser::AST;
use nom::error::Error;

mod parser;
mod generation;

fn main() {
    let result = parser::parse(r#"
        if true {
            /say if
        } else if false {
            /say if else
        } else {
            /say else
        }
    "#).unwrap();
    dbg!(&result);
    generation::generate(result.1);
}

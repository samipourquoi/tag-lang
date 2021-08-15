#![allow(dead_code)]
#![allow(unused_imports)]

use nom_greedyerror::convert_error;
use nom::Finish;
use nom::Err::*;
use crate::errors::CompilerError;

mod parser;
mod generator;
mod errors;

fn main() {
    let input =
    r##"
        def hello(content) {
            /say #{contentt}
        }

        hello(1);
    "##;

    let result = compile(input);

    if let Ok(result) = result {
    } else if let Err(err) = result {
        err.format(input);
    }
}

fn compile(input: &str) -> Result<(), CompilerError> {
    let ast = parser::parse(input).finish()?;
    dbg!(&ast);
    generator::generate(ast.1)
}

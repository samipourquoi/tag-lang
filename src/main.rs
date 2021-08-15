#![allow(dead_code)]
#![allow(unused_imports)]

use nom_greedyerror::convert_error;
use nom::Finish;
use nom::Err::*;

mod parser;
mod generator;
mod errors;

fn main() {
    let input =
    r##"
        def $hello($world) {
            /say #{$world}
        }
    "##;
    let result = parser::parse(input).finish();

    if let Ok(result) = result {
        dbg!(&result);
        generator::generate(result.1);
    } else if
        let Err(err) = result
    {
        // println!("{:?}", err);
        err.format(input);
    }
}

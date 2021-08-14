#![allow(dead_code)]
#![allow(unused_imports)]

use nom_greedyerror::convert_error;

mod parser;
mod generator;

fn main() {
    let input = r##"
        def hello() {
        }
    "##;
    let result = parser::parse(input);

    if let Ok(result) = result {
        dbg!(&result);
        generator::generate(result.1);
    } else if
        let Err(nom::Err::Error(err))
        | Err(nom::Err::Failure(err))
        = result
    {
        println!("{}", convert_error(input, err));
    }
}

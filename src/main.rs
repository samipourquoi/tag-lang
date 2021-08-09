#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r##"
      one := 1;
      $var := 3 * one + 2;
    "##).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

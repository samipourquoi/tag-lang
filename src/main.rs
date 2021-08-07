#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      /say #{1}
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

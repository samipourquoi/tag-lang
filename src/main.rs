#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      hello := 1;

      if true {
        /say #{hello}
      }
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

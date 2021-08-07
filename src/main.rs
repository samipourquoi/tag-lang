#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      $hello := true;
      $world := false;

      if $hello {
        if $world {
          /say yes!!!
        }
      }
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

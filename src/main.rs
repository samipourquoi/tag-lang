#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      def $log(content: string) {
        /say hello world
      }
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

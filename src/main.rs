#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r##"
      def log(content) {
        /say #{content}
      }

      log(1);
      log(2 * 3);
    "##).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

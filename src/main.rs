#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r##"
      def $log(selector, $content) {
        /tellraw #{selector} { "storage": "tag:runtime", "nbt": "vars[-1].content" }
      }

      $log(true, true);
    "##).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

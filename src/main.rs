#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod generator;

fn main() {
    let result = parser::parse(r##"
      def log(content) {
        /say log(content)
      }

      def $log(content) {
        /say $log(content) #{content}
      }

      def $log($content) {
        /say $log($content)
      }

      $var := true;

      log(true);
      $log(true);
      $log($var);
    "##).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

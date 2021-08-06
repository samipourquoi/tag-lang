mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      $hello := true;

      /execute if data storage vars[-1].hello run say hello world
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

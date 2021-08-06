mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
      $hello := 1;
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

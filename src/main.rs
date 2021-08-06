mod parser;
mod generator;

fn main() {
    let result = parser::parse(r#"
        if true {
            /say if
        } else if false {
            /say if else
        } else {
            /say else
        }
    "#).unwrap();
    dbg!(&result);
    generator::generate(result.1);
}

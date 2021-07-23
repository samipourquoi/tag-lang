mod parser;

fn main() {
    let result = parser::parse(r#"
        /say hello world
        /help
        /tellraw @a [{"text":"hello world"},{"score":{"name":"@s","objective":"deaths"}}]

        if 1 == 2 {

        }
    "#);
    dbg!(result);
}

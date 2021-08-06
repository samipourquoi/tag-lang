use crate::parser::ParseResult;

pub enum Function {
    Macro {
        name: String
    },
    Function {
        name: String,
        args: Vec<Argument>
    }
}

pub enum Argument {

}

// pub(in super) fn parse_function(input: &str) -> ParseResult<Function> {
//
// }

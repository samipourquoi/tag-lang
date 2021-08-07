use crate::parser::Statement;
use crate::parser::statement::parse_block;
use nom::combinator::map;
use crate::parser::typing::parse_declaration_typing;
use crate::parser::expression::parse_variable;
use nom::sequence::pair;
use nom::multi::separated_list0;
use crate::parser::delimited;
use crate::parser::typing::Typing;
use crate::parser::expression::VariableName;
use crate::parser::ws;
use crate::parser::identifier;
use nom::bytes::complete::tag;
use crate::parser::ParseResult;
use crate::generator::simplify::IsDynamic;

#[derive(Debug)]
pub enum Function {
    Macro {
        name: String,
        static_args: Vec<Argument>,
        block: Vec<Statement>
    },
    Function {
        name: String,
        dyn_args: Vec<Argument>,
        static_args: Vec<Argument>,
        block: Vec<Statement>
    }
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub var: VariableName,
    pub typing: Typing
}

pub fn parse_function(input: &str) -> ParseResult<Function> {
    let (input, _) = tag("def ")(input)?;
    let (input, fn_name) = ws(parse_variable)(input)?;
    let (input, args) = delimited(ws(tag("(")), separated_list0(
        ws(tag(",")),
        map(pair(parse_variable, parse_declaration_typing),
            |(var, typing)| Argument { var, typing })
    ), ws(tag(")")))(input)?;
    let (input, block) = parse_block(input)?;

    let dyn_args: Vec<Argument> = args.iter()
        .filter(|arg| arg.var.is_dynamic())
        .cloned().collect();
    let static_args: Vec<Argument> = args.iter()
        .filter(|arg| arg.var.is_static())
        .cloned().collect();

    Ok((input, match fn_name {
        VariableName::Dynamic(name) => Function::Function {
            name, dyn_args, static_args, block
        },
        VariableName::Static(name) if dyn_args.is_empty() && block.is_static() => Function::Macro {
            name, static_args, block
        },
        _ => panic!("can't use dynamic args in a macro declaration.")
    }))
}

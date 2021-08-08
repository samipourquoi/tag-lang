use crate::parser::statement::VariableSignature;
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
use crate::generator::staticness::IsStatic;

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub name: String,
    pub dynamic: bool,
    pub static_args: Vec<VariableSignature>,
    pub dyn_args: Vec<VariableSignature>
}

pub fn parse_function(input: &str) -> ParseResult<Function> {
    let (input, _) = tag("def ")(input)?;
    let (input, fn_name) = ws(parse_variable)(input)?;
    let (input, args) = delimited(ws(tag("(")), separated_list0(
        ws(tag(",")),
        map(pair(parse_variable, parse_declaration_typing),
            |(name, typing)| VariableSignature { name, typing })
    ), ws(tag(")")))(input)?;
    let (input, block) = parse_block(input)?;

    let dyn_args: Vec<VariableSignature> = args.iter()
        .filter(|arg| arg.name.is_dynamic())
        .cloned().collect();
    let static_args: Vec<VariableSignature> = args.iter()
        .filter(|arg| arg.name.is_static())
        .cloned().collect();

    let signature = match fn_name {
        VariableName::Dynamic(name) => FunctionSignature {
            dynamic: true, name, dyn_args, static_args,
        },
        VariableName::Static(name) if dyn_args.is_empty() && block.is_static() => FunctionSignature {
            dynamic: false, name, dyn_args, static_args,
        },
        _ => panic!("can't use dynamic args in a macro declaration.")
    };

    Ok((input, Function { signature, block }))
}

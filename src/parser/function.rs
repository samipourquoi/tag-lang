use crate::parser::{Span, Position};
use nom::combinator::cut;
use nom::error::context;
use crate::parser::expression::parse_expression;
use nom::branch::alt;
use crate::parser::expression::Expression;
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
use nom::character::complete::{multispace0, multispace1};
use nom_locate::position;
use crate::errors::CompilerError;

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Vec<Statement>,
    pub position: Position
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub name: VariableName,
    pub args: Vec<VariableSignature>
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: VariableName,
    pub args: Vec<Expression>,
    pub position: Position
}

pub fn parse_function(input: Span) -> ParseResult<Function> {
    let (input, position) = position(input)?;
    let (input, _) = tag("def ")(input)?;
    let (input, name) = cut(ws(parse_variable))(input)?;
    let (input, args) = delimited(ws(tag("(")), cut(separated_list0(
        ws(tag(",")),
        map(pair(parse_variable, parse_declaration_typing),
            |(name, typing)| VariableSignature { name, typing })
    )), ws(tag(")")))(input)?;
    let (input, block) = parse_block(input)?;

    let dyn_args: Vec<VariableSignature> = args.iter()
        .filter(|arg| arg.name.is_dynamic())
        .cloned().collect();

    let signature = match &name {
        VariableName::Dynamic(_) => FunctionSignature {
            name: name.clone(), args,
        },
        VariableName::Static(_) if dyn_args.is_empty() && block.is_static() => FunctionSignature {
            name: name.clone(), args
        },
        _ => return Err(CompilerError::fail(position, "can't use dynamic args in a macro declaration."))
    };

    if name.is_static() && block.is_dynamic() {
        return Err(CompilerError::fail(position, "can't use dynamic statements in a static function"));
    }

    Ok((input, Function { signature, block, position: position.into() }))
}

pub fn parse_function_call(input: Span) -> ParseResult<FunctionCall> {
    let (_, position) = position(input)?;
    let (input, name) = parse_variable(input)?;
    let (input, args) = delimited(
        ws(tag("(")),
        separated_list0(
            ws(tag(",")),
            parse_expression,
        ),
        ws(tag(")"))
    )(input)?;

    if name.is_static() && args.iter().any(|arg| arg.is_dynamic()) {
        panic!("can't call a static function with dynamic arguments")
    }

    Ok((input, FunctionCall {
        name, args, position: position.into()
    }))
}

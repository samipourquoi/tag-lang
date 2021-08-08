use nom::bytes::complete::take_until;
use nom::combinator::into;
use nom::multi::many1;
use nom::character::is_newline;
use nom::combinator::verify;
use nom::character::complete::anychar;
use nom::sequence::pair;
use crate::parser::end_of_line;
use crate::parser::function::parse_function;
use crate::parser::function::Function;
use crate::parser::{ParseResult, ws, read_line};
use nom::branch::alt;
use nom::combinator::{map, success};
use crate::parser::expression::{Expression, VariableName, parse_expression, parse_variable};
use nom::sequence::{preceded, delimited, terminated};
use nom::bytes::complete::tag;
use nom::multi::many0;
use crate::parser::typing::{Typing, parse_declaration_typing};
use std::iter::FromIterator;
use crate::generator::staticness::IsStatic;
use nom::character::complete::alphanumeric0;

#[derive(Debug)]
pub enum Statement {
    Command(Command),
    IfStatement(IfStatement),
    VariableAssignment(VariableAssignment),
    FunctionDeclaration(Function)
}

#[derive(Debug)]
pub struct IfStatement {
    pub expr: Expression,
    pub block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>,
    pub else_if: Box<Option<IfStatement>>
}

#[derive(Debug)]
pub struct Command {
    pub start: Vec<(String, Expression)>,
    pub end: String
}

#[derive(Debug)]
pub struct VariableSignature {
    pub name: VariableName,
    pub typing: Typing
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub signature: VariableSignature,
    pub value: Expression
}

pub(in super) fn parse_block(input: &str) -> ParseResult<Vec<Statement>> {
    delimited(
        ws(tag("{")),
        many0(ws(parse_statement)),
        ws(tag("}"))
    )(input)
}

pub(in super) fn parse_statement(input: &str) -> ParseResult<Statement> {
    alt((
        map(parse_command,
            |cmd| Statement::Command(cmd)),
        map(parse_if_statement,
            |if_stmt| Statement::IfStatement(if_stmt)),
        map(terminated(parse_variable_declaration, ws(tag(";"))),
            |var| Statement::VariableAssignment(var)),
        map(parse_function, 
            |function| Statement::FunctionDeclaration(function))
    ))(input)
}

pub(in super) fn parse_if_statement(input: &str) -> ParseResult<IfStatement> {
    let (input, expr) = preceded(tag("if "), ws(parse_expression))(input)?;
    let (input, block) = parse_block(input)?;

    // This basically transforms this...
    // > if A {
    // >   ...
    // > } else if B {
    // >   ...
    // > } else {
    //
    // ...into this:
    // > if A {
    // >   ...
    // > } else {
    // >   if B {
    // >     ...
    // >   } else {
    // >     ...
    // >   }
    // > }
    if let Ok((input, else_if))
    = preceded(ws(tag("else ")), parse_if_statement)(input)
    {
        Ok((input, IfStatement {
            expr,
            block,
            else_if: Box::new(Some(else_if)),
            else_block: None
        }))
    } else if let Ok((input, else_block))
    = preceded(ws(tag("else")), parse_block)(input)
    {
        Ok((input, IfStatement {
            expr,
            block,
            else_if: Box::new(None),
            else_block: Some(else_block)
        }))
    } else {
        Ok((input, IfStatement {
            expr,
            block,
            else_if: Box::new(None),
            else_block: None
        }))
    }
}

pub fn parse_command(input: &str) -> ParseResult<Command> {
    let (input, _) = tag("/")(input)?;
    let (input, start) = many0(
        pair(
            map(
                verify(
                    take_until("#{"), 
                    |string: &str| string.chars().all(|c| !is_newline(c as u8))
                ), 
                str::to_string
            ),
            delimited(tag("#{"), ws(parse_expression), tag("}"))
        )
    )(input)?;
    let (input, end) = read_line(input)?;

    if start.iter().any(|(_, expr)| expr.is_dynamic()) {
        panic!("can't interpolate a dynamic value in a command");
    }

    Ok((input, Command { start, end }))
}

pub(in super) fn parse_variable_declaration(input: &str)
    -> ParseResult<VariableAssignment>
{
    let (input, name) = parse_variable(input)?;
    let (input, typing) = ws(parse_declaration_typing)(input)?;
    let (input, value) = preceded(ws(tag(":=")), parse_expression)(input)?;

    Ok((input, VariableAssignment {
        value, signature: VariableSignature { name, typing }
    }))
}

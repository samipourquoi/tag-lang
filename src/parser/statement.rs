use crate::parser::{ParseResult, ws, read_line};
use nom::branch::alt;
use nom::combinator::{map, success};
use crate::parser::expression::{Expression, VariableName, parse_expression, parse_variable};
use nom::sequence::{preceded, delimited, terminated};
use nom::bytes::complete::tag;
use nom::multi::many0;
use crate::parser::typing::{Typing, parse_declaration_typing};

#[derive(Debug)]
pub enum Statement {
    Command(Command),
    IfStatement(IfStatement),
    VariableAssignment(VariableAssignment)
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
    pub value: String
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub var: VariableName,
    pub value: Expression,
    pub typing: Typing
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
            |var| Statement::VariableAssignment(var))
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

pub(in super) fn parse_command(input: &str) -> ParseResult<Command> {
    map(preceded(tag("/"), read_line),
        |cmd| Command { value: cmd })(input)
}

pub(in super) fn parse_variable_declaration(input: &str)
    -> ParseResult<VariableAssignment>
{
    let (input, var) = parse_variable(input)?;
    let (input, typing) = ws(parse_declaration_typing)(input)?;
    let (input, value) = preceded(ws(tag(":=")), parse_expression)(input)?;

    Ok((input, VariableAssignment {
        var, value, typing
    }))
}

pub mod expression;
pub mod statement;
pub mod function;
pub mod typing;

use nom::IResult;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::map;
use nom::character::complete::{digit1, not_line_ending, line_ending as eol, multispace0, alpha0, alpha1, alphanumeric1, alphanumeric0};
use nom::error::ParseError;
use crate::parser::statement::{Statement, parse_statement};

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>
}

type ParseResult<'a, T> = IResult<&'a str, T>;

pub fn parse(input: &str) -> ParseResult<AST> {
    let (input, statements) = many0(ws(parse_statement))(input)?;

    Ok((input, AST {
        statements
    }))
}

fn identifier(input: &str) -> ParseResult<String> {
    let (input, first) = alpha1(input)?;
    let (input, second) = alphanumeric0(input)?;
    let (input, third) = many0(tag("'"))(input)?;

    Ok((input, first.to_string() + second + &third.join("")))
}

fn read_line(input: &str) -> ParseResult<String> {
    let (input, line) = terminated(not_line_ending, eol)(input)?;
    Ok((input, line.to_string()))
}

fn end_of_line(input: &str) -> ParseResult<&str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        eol(input)
    }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}

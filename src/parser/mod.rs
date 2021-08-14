pub mod expression;
pub mod statement;
pub mod function;
pub mod typing;

use nom_locate::LocatedSpan;
use nom::error::ErrorKind;
use nom::combinator::all_consuming;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{delimited, terminated};
use nom::bytes::complete::tag;
use nom::character::complete::{not_line_ending, line_ending as eol, multispace0, alpha1, alphanumeric0};
use nom::error::ParseError;
use crate::parser::statement::{Statement, parse_statement};
use nom_greedyerror::GreedyError;

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>
}

type Span<'a> = LocatedSpan<&'a str>;

type ParseResult<'a, T> = IResult<Span<'a>, T, GreedyError<Span<'a>, ErrorKind>>;

pub fn parse(input: Span) -> ParseResult<AST> {
    let (input, statements) = all_consuming(many0(ws(parse_statement)))(input)?;

    Ok((input, AST {
        statements
    }))
}

fn identifier(input: Span) -> ParseResult<String> {
    let (input, first) = alpha1(input)?;
    let (input, second) = alphanumeric0(input)?;
    let (input, third) = many0(tag("'"))(input)?;

    Ok((input, first.to_string() + second + &third.join("")))
}

fn read_line(input: Span) -> ParseResult<String> {
    let (input, line) = terminated(not_line_ending, eol)(input)?;
    Ok((input, line.to_string()))
}

fn end_of_line(input: Span) -> ParseResult<Span> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        eol(input)
    }
}

fn ws<'a, T, F>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<T>
    where
        F: Fn(Span<'a>) -> ParseResult<T>
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}

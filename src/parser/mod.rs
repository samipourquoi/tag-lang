pub mod expression;
pub mod statement;
pub mod function;
pub mod typing;
mod shunting_yard;

use nom_locate::LocatedSpan;
use nom::error::ErrorKind;
use nom::combinator::{all_consuming, verify};
use nom::{IResult, Offset, Parser};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::bytes::complete::tag;
use nom::character::complete::{not_line_ending, line_ending as eol, multispace0, alpha1, alphanumeric0, one_of, anychar};
use nom::error::ParseError;
use crate::parser::statement::{Statement, parse_statement};
use nom_greedyerror::GreedyError;
use crate::errors::CompilerError;
use nom::Err;
use std::ops::Add;
use nom::branch::alt;
use nom::character::is_alphanumeric;

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>
}

impl Add for AST {
    type Output = Self;

    fn add(self, other: AST) -> Self::Output {
        AST {
            statements: vec![self.statements, other.statements].concat()
        }
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl From<Span<'_>> for Position {
    fn from(span: Span) -> Self {
        Position {
            offset: span.location_offset(),
            length: span.fragment().len(),
            line: span.location_line() as usize,
            column: span.get_column()
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { offset: 0, length: 0, line: 0, column: 0 }
    }
}

type ParseResult<'a, T> = IResult<Span<'a>, T, CompilerError>;

pub fn parse(input: &str) -> ParseResult<AST> {
    let input = Span::new(input);
    let (input, statements) = all_consuming(many0(ws(parse_statement)))(input)?;

    Ok((input, AST {
        statements
    }))
}

fn identifier(input: Span) -> ParseResult<String> {
    err_msg("invalid identifier", |input| {
        let (input, first) = alpha1(input)?;
        let (input, second) = many0(alt((
            one_of("_'"),
            verify(anychar, |char| is_alphanumeric(*char as u8))
        )))(input)?;

        Ok((input, first.fragment().to_string() + second.iter().collect::<String>().as_str()))
    })(input)
}

fn read_line(input: Span) -> ParseResult<String> {
    let (input, line) = terminated(not_line_ending, eol)(input)?;
    Ok((input, line.fragment().to_string()))
}

fn end_of_line(input: Span) -> ParseResult<Span> {
    if input.is_empty() {
        Ok((input.clone(), input))
    } else {
        eol(input)
    }
}

fn ws<'a, T, F>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<T>
    where F: Fn(Span<'a>) -> ParseResult<T>
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}

pub fn err_msg<'a, S, T, F>(msg: S, mut parser: F)
    -> impl FnMut(Span<'a>) -> ParseResult<T>
    where S: ToString,
          F: FnMut(Span<'a>) -> ParseResult<T>
{
    move |input| {
        parser(input).map_err(|err|
           err.map(|comp_err| CompilerError {
               error: msg.to_string(),
               position: comp_err.position
           })
        )
    }
}

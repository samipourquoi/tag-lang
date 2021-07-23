use nom::{
    bytes::complete::take_until,
    IResult,
    multi::many0,
    combinator::opt,
    bytes::complete::tag,
    character::{
        complete::{
            anychar,
            multispace0
        },
        complete::line_ending as eol
    },
    branch::alt,
    sequence::{terminated, delimited},
    combinator::eof,
    multi::many1,
    character::complete::{alphanumeric1, alphanumeric0}
};
use nom::character::complete::not_line_ending;

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    Command(Command)
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub value: String
}

type ParseResult<'a, T> = IResult<&'a str, T>;

pub fn parse(input: &str) -> ParseResult<AST> {
    let (input, statements) = many0(
        delimited(multispace0, parse_statement, multispace0)
    )(input)?;

    Ok((input, AST {
        statements
    }))
}

fn parse_statement(input: &str) -> ParseResult<Statement> {
    let (input, command) = parse_command(input)?;
    Ok((input, Statement::Command(command)))
}

fn parse_command(input: &str) -> ParseResult<Command> {
    let (input, _) = tag("/")(input)?;
    let (input, command) = read_line(input)?;

    Ok((input, Command { value: command.to_string() }))
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

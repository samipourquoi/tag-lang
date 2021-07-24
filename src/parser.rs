use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::branch::alt;
use nom::character::complete::line_ending as eol;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::anychar;
use nom::bytes::complete::take_until;
use nom::{IResult, InputTake, UnspecializedInput};
use nom::multi::many0;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::multi::many1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::alphanumeric0;
use nom::Parser;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::InputLength;
use nom::InputIter;
use nom::sequence::preceded;
use nom::error::ParseError;

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

#[derive(Debug)]
pub enum Statement {
    Command(Command)
}

fn parse_statement(input: &str) -> ParseResult<Statement> {
    // alt((
    //     map(parse_command, |cmd| Statement::Command(cmd)),
    // ))(input)
    map(parse_command, |cmd| Statement::Command(cmd))(input)
}

#[derive(Debug)]
pub struct Command {
    pub value: String
}

fn parse_command(input: &str) -> ParseResult<Command> {
    map(preceded(tag("/"), read_line),
        |cmd| Command { value: cmd })(input)
}

#[derive(Debug)]
enum Expression {
    Sum(Summand, Box<Expression>),
    Summand(Summand)
}

fn parse_expression(input: &str) -> ParseResult<Expression> {
    alt((
        map(separated_pair(parse_summand, ws(tag("+")), parse_expression),
            |(summand, expression)| Expression::Sum(summand, Box::new(expression))),

        map(parse_summand, |summand| Expression::Summand(summand))
    ))(input)
}

#[derive(Debug)]
enum Summand {
    Multiplication(Term, Box<Summand>),
    Term(Term)
}

fn parse_summand(input: &str) -> ParseResult<Summand> {
    alt((
        |input| {
            let (input, term) = parse_term(input)?;
            let (input, _) = ws(tag("*"))(input)?;
            let (input, summand) = parse_summand(input)?;

            Ok((input, Summand::Multiplication(term, Box::new(summand))))
        },
        map(parse_term, |term| Summand::Term(term))
    ))(input)
}

#[derive(Debug)]
enum Term {
    Number(i32),
    Expression(Box<Expression>)
}

fn parse_term(input: &str) -> ParseResult<Term> {
    alt((
        map(digit1,
            |d: &str| Term::Number(d.to_string().parse().unwrap())),

        delimited(ws(tag("(")),
                  map(parse_expression, |expr| Term::Expression(Box::new(expr))),
                  ws(tag(")")))
    ))(input)
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

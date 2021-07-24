use nom::sequence::{delimited, delimitedc};
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

fn parse_block(input: &str) -> ParseResult<Vec<Statement>> {
    delimited(
        ws(tag("{")),
        many0(ws(parse_statement)),
        ws(tag("}"))
    )(input)
}

#[derive(Debug)]
pub enum Statement {
    Command(Command),
    IfStatement(IfStatement)
}

fn parse_statement(input: &str) -> ParseResult<Statement> {
    alt((
        map(parse_command, |cmd| Statement::Command(cmd)),
        map(parse_if_statement, |if_stmt| Statement::IfStatement(if_stmt))
    ))(input)
}

#[derive(Debug)]
pub struct IfStatement {
    pub expr: Expression,
    pub block: Vec<Statement>
}

fn parse_if_statement(input: &str) -> ParseResult<IfStatement> {
    let (input, expr) = preceded(tag("if "), ws(parse_expression))(input)?;
    let (input, block) = parse_block(input)?;

    Ok((input, IfStatement { expr, block }))
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
pub enum Expression {
    Sum(Summand, Box<Expression>),
    Summand(Summand),
    Boolean(bool)
}

fn parse_expression(input: &str) -> ParseResult<Expression> {
    alt((
        map(separated_pair(parse_summand, ws(tag("+")), parse_expression),
            |(summand, expression)| Expression::Sum(summand, Box::new(expression))),

        map(parse_summand, |summand| Expression::Summand(summand)),

        map(tag("true"), |_| Expression::Boolean(true)),
        map(tag("false"), |_| Expression::Boolean(false))
    ))(input)
}

#[derive(Debug)]
pub enum Summand {
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
pub enum Term {
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

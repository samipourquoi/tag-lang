use crate::parser::function::parse_function_call;
use crate::parser::function::FunctionCall;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::{separated_pair, delimited};
use crate::parser::{ws, ParseResult, identifier};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use crate::parser::typing::parse_typing;
use nom::Parser;

#[derive(Debug, Clone)]
pub enum Expression {
    Sum(Summand, Box<Expression>),
    Summand(Summand),
    Boolean(bool),
    Variable(VariableName)
}

#[derive(Debug, Clone)]
pub enum Summand {
    Multiplication(Term, Box<Summand>),
    Term(Term)
}

#[derive(Debug, Clone)]
pub enum Term {
    Number(i32),
    FunctionCall(FunctionCall),
    Expression(Box<Expression>)
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum VariableName {
    Dynamic(String),
    Static(String)
}

impl VariableName {
    pub fn get_name(&self) -> &String {
        match self {
            VariableName::Dynamic(name) => name,
            VariableName::Static(name) => name
        }
    }
}

pub(in super) fn parse_expression(input: &str) -> ParseResult<Expression> {
    alt((
        map(separated_pair(parse_summand, ws(tag("+")), parse_expression),
            |(summand, expression)| Expression::Sum(summand, Box::new(expression))),

        map(parse_summand, |summand| Expression::Summand(summand)),

        map(tag("true"), |_| Expression::Boolean(true)),
        map(tag("false"), |_| Expression::Boolean(false)),

        map(parse_variable, |var| Expression::Variable(var))
    ))(input)
}

pub(in super) fn parse_summand(input: &str) -> ParseResult<Summand> {
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

pub(in super) fn parse_term(input: &str) -> ParseResult<Term> {
    alt((
        map(digit1,
            |d: &str| Term::Number(d.to_string().parse().unwrap())),

        map(parse_function_call,
            |call| Term::FunctionCall(call)),

        delimited(ws(tag("(")),
                  map(parse_expression, |expr| Term::Expression(Box::new(expr))),
                  ws(tag(")")))
    ))(input)
}

pub(in super) fn parse_variable(input: &str) -> ParseResult<VariableName> {
    opt(tag("$"))(input).and_then(|(input, dollar)|
        map(identifier, |name|
            match dollar {
                Some(_) => VariableName::Dynamic(name),
                None    => VariableName::Static(name)
            }
        )(input)
    )
}

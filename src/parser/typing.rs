use crate::parser::{ParseResult, ws};
use nom::branch::alt;
use nom::combinator::{map, opt, map_res, success};
use nom::bytes::complete::tag;
use nom::sequence::preceded;

#[derive(Clone, Debug)]
pub enum Typing {
    Integer,
    String,
    Boolean,
    Unknown
}

pub(in super) fn parse_typing(input: &str) -> ParseResult<Typing> {
    alt((
        map(tag("int"), |_| Typing::Integer),
        map(tag("string"), |_| Typing::String),
    ))(input)
}

pub(in super) fn parse_declaration_typing(input: &str) -> ParseResult<Typing> {
    alt((
        preceded(ws(tag(":")), ws(parse_typing)),
        success(Typing::Unknown)
    ))(input)
}

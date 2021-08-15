use crate::parser::{Span, err_msg};
use crate::parser::{ParseResult, ws};
use nom::branch::alt;
use nom::combinator::{map, opt, map_res, success, cut};
use nom::bytes::complete::tag;
use nom::sequence::preceded;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typing {
    Integer,
    String,
    Boolean,
    Unknown
}

pub(in super) fn parse_typing(input: Span) -> ParseResult<Typing> {
    err_msg("invalid type", alt((
        map(tag("int"), |_| Typing::Integer),
        map(tag("string"), |_| Typing::String),
    )))(input)
}

pub(in super) fn parse_declaration_typing(input: Span) -> ParseResult<Typing> {
    alt((
        preceded(ws(tag(":")), ws(parse_typing)),
        success(Typing::Unknown)
    ))(input)
}

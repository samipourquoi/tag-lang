use nom_greedyerror::GreedyErrorKind;
use nom::error::{ParseError, ErrorKind};
use crate::parser::{Position, Span};
use nom::Parser;

#[derive(Debug)]
pub struct CompilerError {
    pub error: String,
    pub position: Position
}

impl<'a, S: ToString> From<(Span<'a>, S)> for CompilerError {
    fn from(from: (Span<'a>, S)) -> Self {
        let position: Position = from.0.into();
        let error: String = from.1.to_string();

        CompilerError { error, position }
    }
}

impl CompilerError {
    pub fn fail<S: ToString>(input: Span, error: S) -> nom::Err<Self> {
        nom::Err::Failure((input, error).into())
    }
}

impl ParseError<Span<'_>> for CompilerError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        dbg!(&kind);
        CompilerError {
            error: match kind {
                _ => "invalid syntax"
            }.into(),
            position: input.into()
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }
}

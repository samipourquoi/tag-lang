use nom_greedyerror::GreedyErrorKind;
use nom::error::{ParseError, ErrorKind};
use crate::parser::{Position, Span};
use nom::Parser;
use std::fmt;
use nom_locate::position;

#[derive(Debug)]
pub struct CompilerError {
    pub error: String,
    pub position: Position
}

impl<'a, S: ToString> From<(Span<'a>, S)> for CompilerError {
    fn from((span, msg): (Span<'a>, S)) -> Self {
        (Position::from(span), msg).into()
    }
}

impl<'a, S: ToString> From<(Position, S)> for CompilerError {
    fn from(from: (Position, S)) -> Self {
        let position = from.0;
        let error: String = from.1.to_string();

        CompilerError { error, position }
    }
}

impl CompilerError {
    pub fn fail<P: Into<Position>, S: ToString>(pos: P, error: S) -> nom::Err<Self> {
        nom::Err::Failure((pos.into(), error).into())
    }

    pub fn format(&self, src: &str) {
        use termion::{color, color::Fg, style};

        const OFFSET: i32 = 2;

        let lines: Vec<_> = src.split("\n").collect();
        let line = self.position.line as i32;
        let lines_nb: Vec<_> = (OFFSET-3..OFFSET+2)
            .map(|off| off+line-1)
            .filter_map(|i| if i <= 0 || lines.len() <= i as usize { None } else { Some(i) })
            .collect();
        let margin = lines_nb.iter()
            .map(|i| i.to_string().len())
            .max().unwrap_or(0);

        let mut out: Vec<String> = vec![];
        for i in lines_nb {
            out.push(format!("{}{: >margin$} |{} {}",
                    Fg(color::Red), i, Fg(color::Reset),
                    lines[(i - 1) as usize],
                    margin = margin));

            if i == self.position.line as i32 {
                out.push(format!("{}{: >margin$} ){}{: >column$}{}{}",
                                 Fg(color::Red), " ", Fg(color::LightRed), "^", "here", style::Reset,
                                 margin = margin, column = self.position.column))
            }
        }
        out.push(format!("{}{}error{}: {}",
                         style::Bold, Fg(color::Red), style::Reset,
                         self.error));

        println!("{}", out.join("\n"));
    }
}

impl ParseError<Span<'_>> for CompilerError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
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

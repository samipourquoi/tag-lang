use crate::parser::{Span, ParseResult, ws};
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::pair;
use std::cmp::Ordering;
use nom::character::complete::digit1;
use std::str::FromStr;
use crate::errors::CompilerError;

macro_rules! precedence {
    (enum $name:ident {
        $($key:ident => $value:expr),*
    }) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        enum $name {
            $($key),*
        }

        impl Operator {
            fn get_precedence(&self) -> i32 {
                match self {
                    $($key => $value),*
                }
            }
        }
    }
}

// We use the same operator precedence as Ruby.
//https://stackoverflow.com/questions/21060234/ruby-operator-precedence-table
// The higher it is, the sooner it will be evaluated.
precedence!(enum Operator {
    Sentinel => 0,

    Not => 1,
    Neg => 3,
    Mult => 4,
    Div => 4,
    Plus => 5,
    Minus => 5,
    Eq => 10,
    Neq => 10,
    And => 11,
    Or => 12,
    NotNot => 21,
    AndAnd => 22,
    OrOr => 22
});

impl Operator {
    fn is_binary(&self) -> bool {
        !self.is_unary() && !self.is_sentinel()
    }

    fn is_unary(&self) -> bool {
        match self {
            Operator::Neg => true,
            Operator::Not => true,
            _ => false
        }
    }

    fn is_sentinel(&self) -> bool {
        match self {
            Operator::Sentinel => true,
            _ => false
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum Tree {
    Unary(Operator, Box<Tree>),
    Binary(Box<Tree>, Operator, Box<Tree>),
    Leaf(Value)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Number(i32)
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_precedence().cmp(&other.get_precedence())
    }
}

fn binary(input: Span) -> ParseResult<Operator> {
    alt((
        map(ws(tag("+")), |_| Operator::Plus),
        map(ws(tag("-")), |_| Operator::Minus),
        map(ws(tag("*")), |_| Operator::Mult),
        map(ws(tag("/")), |_| Operator::Div),

        map(ws(tag("&&")), |_| Operator::And),
        map(ws(tag("||")), |_| Operator::Or),
        map(ws(tag("==")), |_| Operator::Eq),
        map(ws(tag("!=")), |_| Operator::Neq),
        map(ws(tag("and")), |_| Operator::AndAnd),
        map(ws(tag("or")), |_| Operator::OrOr),
    ))(input)
}

fn unary(input: Span) -> ParseResult<Operator> {
    alt((
        map(ws(tag("-")), |_| Operator::Neg),
        map(ws(tag("!")), |_| Operator::Not),
        map(ws(tag("not")), |_| Operator::NotNot),
    ))(input)
}

/// We don't want [Operator::AndAnd] and [Operator::And]
/// to be differentiated when we will traverse the AST,
/// so this function takes care of that.
fn convert_operator(op: Operator) -> Operator {
    match op {
        Operator::AndAnd => Operator::And,
        Operator::OrOr => Operator::Or,
        Operator::NotNot => Operator::Not,
        _ => op
    }
}

fn value(input: Span) -> ParseResult<Value> {
    map(digit1, |i: Span| Value::Number(i32::from_str(i.fragment()).unwrap()))(input)
}

fn p<'a>(
    mut input: Span<'a>,
    operators: &mut Vec<Operator>,
    operands: &mut Vec<Tree>
) -> ParseResult<'a, ()> {
    if let Ok((input2, value)) = value(input) {
        input = input2;
        operands.push(Tree::Leaf(value));
    } else if let Ok((input2, _)) = tag::<_, _, CompilerError>("(")(input) {
        operators.push(Operator::Sentinel);
        let (input2, _) = e(input2, operators, operands)?;
        let (input2, _) = tag(")")(input2)?;
        operators.pop();
        input = input2;
    } else if let Ok((input2, op)) = unary(input) {
        push_operator(input2, op, operators, operands);
        let (input2, _) = p(input2, operators, operands)?;
        input = input2;
    } else {
        return Err(CompilerError::fail(input, "invalid expr syntax"));
    }

    Ok((input, ()))
}

fn e<'a>(
    mut input: Span<'a>,
    operators: &mut Vec<Operator>,
    operands: &mut Vec<Tree>
) -> ParseResult<'a, ()> {
    let (mut input, _) = p(input, operators, operands)?;

    while let Ok((input2, op)) = binary(input) {
        push_operator(input2, op, operators, operands)?;
        let (input2, _) = p(input2, operators, operands)?;
        input = input2;
    }
    while !operators.last().ok_or(CompilerError::syntax_error(input))?.is_sentinel() {
        pop_operator(input, operators, operands)?;
    }

    Ok((input, ()))
}

fn push_operator<'a>(
    input: Span<'a>,
    operator: Operator,
    operators: &mut Vec<Operator>,
    operands: &mut Vec<Tree>
) -> ParseResult<'a, ()> {
    while operators.last().ok_or(CompilerError::syntax_error(input))? > &operator {
        pop_operator(input, operators, operands);
    }
    operators.push(convert_operator(operator));

    Ok((input, ()))
}

fn pop_operator<'a,>(
    input: Span<'a>,
    operators: &mut Vec<Operator>,
    operands: &mut Vec<Tree>
) -> ParseResult<'a, ()> {
    if operators.last().ok_or(CompilerError::syntax_error(input))?.is_binary() {
        let tree2 = operands.pop().ok_or(CompilerError::syntax_error(input))?;
        let tree1 = operands.pop().ok_or(CompilerError::syntax_error(input))?;
        let operator = operators.pop().ok_or(CompilerError::syntax_error(input))?;
        operands.push(Tree::Binary(Box::new(tree1), operator, Box::new(tree2)));
    } else {
        let operator = operators.pop().ok_or(CompilerError::syntax_error(input))?;
        let tree = operands.pop().ok_or(CompilerError::syntax_error(input))?;
        operands.push(Tree::Unary(operator, Box::new(tree)));
    }

    Ok((input, ()))
}

/// The algorithm is taken from:
/// https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm
fn shunting_yard(input: Span) -> ParseResult<Tree> {
    let mut operators: Vec<Operator> = vec![Operator::Sentinel];
    let mut operands: Vec<Tree> = vec![];

    let (input, _) = e(input, &mut operators, &mut operands)?;
    operands.last()
        .map(|op| (input, op.clone()))
        .ok_or(CompilerError::syntax_error(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tree = shunting_yard(Span::new("(1 && 2) || 3"));
        println!("{:?}", tree);
    }
}

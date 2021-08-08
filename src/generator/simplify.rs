use crate::parser::expression::Summand;
use crate::parser::expression::Term;
use crate::parser::expression::Expression;
use crate::generator::Generator;

pub trait Simplify<T> {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<T, &'a str> {
        Err("type error, can't resolve")
    }
}

impl Simplify<String> for Expression {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<String, &'a str> {
        let to_i32: i32 = self.simplify(ctx)?;
        Ok(to_i32.to_string())
    }
}

impl Simplify<i32> for Expression {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> {
        match self {
            Expression::Sum(summand, expr) => {
                let a: i32 = summand.simplify(ctx)?;
                let b: i32 = expr.simplify(ctx)?;
                Ok(a + b)
            },
            Expression::Summand(summand) => summand.simplify(ctx),
            Expression::Boolean(_) => Err("can't resolve a boolean into an i32"),
            Expression::Variable(var) => {
                let expr = ctx.get_static_variable_value(var).expect("unknown variable");
                expr.simplify(ctx)
            }
        }
    }
}

impl Simplify<i32> for Summand {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> { 
        match self {
            Summand::Multiplication(term, summand) => Ok(term.simplify(ctx)? * summand.simplify(ctx)?),
            Summand::Term(term) => term.simplify(ctx)
        }
    }
}

impl Simplify<i32> for Term {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> { 
        match self {
            Term::Number(n) => Ok(*n),
            Term::Expression(expr) => expr.simplify(ctx)
        }
    }
}

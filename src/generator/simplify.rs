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
        let as_i32 = self.simplify(ctx).map(|i: i32| i.to_string());
        let as_bool = self.simplify(ctx).map(|bl: bool| bl.to_string());

        as_i32.or(as_bool)
    }
}

impl Simplify<i32> for Expression {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> {
        match self {
            Expression::Sum(summand, expr, _) =>
                Ok(<Summand as Simplify<i32>>::simplify(summand, ctx)? +
                    <Expression as Simplify<i32>>::simplify(expr, ctx)?),
            Expression::Summand(summand, _) => summand.simplify(ctx),
            Expression::Boolean(_, _) => Err("can't resolve a boolean into an i32")
        }
    }
}

impl Simplify<i32> for Summand {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> { 
        match self {
            Summand::Multiplication(term, summand) => 
                Ok(<Term as Simplify<i32>>::simplify(term, ctx)? *
                    <Summand as Simplify<i32>>::simplify(summand, ctx)?),
            Summand::Term(term) => term.simplify(ctx)
        }
    }
}

impl Simplify<i32> for Term {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<i32, &'a str> { 
        match self {
            Term::Number(n) => Ok(*n),
            Term::Expression(expr) => expr.simplify(ctx),
            Term::Variable(var) => ctx.get_static_variable_value(var)
                .expect("unknown variable")
                .simplify(ctx),
            Term::FunctionCall(call) => todo!()
        }
    }
}

impl Simplify<bool> for Expression {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<bool, &'a str> {
        match self {
            Expression::Boolean(bl, _) => Ok(*bl),
            Expression::Summand(summand, _) => summand.simplify(ctx),
            _ => Err("can't resolve to a boolean")
        }
    }
}

impl Simplify<bool> for Summand {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<bool, &'a str> {
        match self {
            Summand::Term(term) => term.simplify(ctx),
            _ => Err("can't convert to a bool")
        }
    }
}

impl Simplify<bool> for Term {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<bool, &'a str> { 
        match self {
            Term::Expression(expr) => expr.simplify(ctx),
            Term::Variable(var) => {
                let expr = ctx.get_static_variable_value(var).expect("unknown variable");
                expr.simplify(ctx)
            },
            _ => Err("can't convert to bool")
        }
    }
}

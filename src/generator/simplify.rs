use crate::parser::expression::Summand;
use crate::parser::expression::Term;
use crate::parser::expression::Expression;
use crate::generator::Generator;
use crate::errors::CompilerError;

pub trait Simplify<T> {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<T, &'a str>;
}

/// While [Simplify<String>] and this impl. sound like they would do
/// the same thing, but they actually don't. [Simplify<String>] will
/// simplify the value if it is a string, whereas this impl. will
/// convert the simplified value to a string.
impl Expression {
    pub fn to_string(&self, ctx: &Generator) -> Result<String, CompilerError> {
        let as_string = self.simplify(ctx).map(|str: String| format!("\"{}\"", str));
        let as_i32 = self.simplify(ctx).map(|i: i32| i.to_string());
        let as_bool = self.simplify(ctx).map(|bl: bool| bl.to_string());

        as_string.or(as_i32).or(as_bool)
            .map_err(|err| CompilerError {
                error: err.to_string(),
                position: self.pos().clone()
            })
    }
}

impl Simplify<String> for Expression {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<String, &'a str> {
        match self {
            Expression::Sum(summand, expr, _) => {
                let str1: String = summand.simplify(ctx)?;
                let str2: String = expr.simplify(ctx)?;
                Ok(str1 + str2.as_str())
            }
            Expression::Summand(summand, _) => summand.simplify(ctx),
            Expression::Boolean(_, _) => Err("can't convert a boolean to a string")
        }
    }
}

impl Simplify<String> for Summand {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<String, &'a str> {
        match self {
            Summand::Multiplication(_, _) => Err("can't multiply a string"),
            Summand::Term(term) => term.simplify(ctx)
        }
    }
}

impl Simplify<String> for Term {
    fn simplify<'a>(&self, ctx: &'a Generator) -> Result<String, &'a str> {
        match self {
            Term::String(str) => Ok(str.clone()),
            Term::FunctionCall(_) => Err("todo: function call -> string"),
            Term::Variable(var) => ctx.get_static_variable_value(var)
                .ok_or("unknown variable")?
                .simplify(ctx),
            Term::Expression(expr) => expr.simplify(ctx),
            Term::Number(_) => Err("can't convert a number to a string")
        }
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
                .ok_or("unknown variable")?
                .simplify(ctx),
            Term::FunctionCall(call) => Err("todo"),
            Term::String(_) => Err("can't convert a string to an i32")
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
                let expr = ctx.get_static_variable_value(var).ok_or("unknown variable")?;
                expr.simplify(ctx)
            },
            _ => Err("can't convert to bool")
        }
    }
}

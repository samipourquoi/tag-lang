use crate::generator::Generator;
use crate::parser::expression::{Expression, Term, Summand, VariableName};
use crate::parser::statement::VariableAssignment;

pub trait IsDynamic {
    fn is_dynamic(&self) -> bool;
    fn is_static(&self) -> bool { !self.is_dynamic() }
}

impl IsDynamic for Expression {
    fn is_dynamic(&self) -> bool {
        match self {
            Expression::Sum(summand, expr) => summand.is_dynamic() && expr.is_dynamic(),
            Expression::Summand(summand) => summand.is_dynamic(),
            Expression::Boolean(_) => false,
            Expression::Variable(var) => var.is_dynamic()
        }
    }
}

impl IsDynamic for Summand {
    fn is_dynamic(&self) -> bool {
        match self {
            Summand::Multiplication(term, summand) => term.is_dynamic() && summand.is_dynamic(),
            Summand::Term(term) => term.is_dynamic()
        }
    }
}

impl IsDynamic for Term {
    fn is_dynamic(&self) -> bool {
        match self {
            Term::Number(_) => false,
            Term::Expression(expr) => expr.is_dynamic()
        }
    }
}

impl IsDynamic for VariableAssignment {
    fn is_dynamic(&self) -> bool {
        self.var.is_dynamic()
    }
}

impl IsDynamic for VariableName {
    fn is_dynamic(&self) -> bool {
        match self {
            VariableName::Dynamic(_) => true,
            VariableName::Static(_) => false
        }
    }
}

impl Generator {
    pub fn simplify_expression(&self, expression: &Expression) -> Expression {
        // TODO
        expression.clone()
    }
}

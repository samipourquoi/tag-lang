use crate::parser::function::Function;
use crate::parser::statement::IfStatement;
use crate::parser::statement::Statement;
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

impl IsDynamic for Statement {
    fn is_dynamic(&self) -> bool { 
        match self {
            Statement::IfStatement(if_stmt) => if_stmt.is_dynamic(),
            Statement::Command(_) => false,
            Statement::VariableAssignment(var) => var.is_dynamic(),
            Statement::FunctionDeclaration(function) => function.is_dynamic(),
            _ => false
        }
    }
}

impl IsDynamic for Vec<Statement> {
    fn is_dynamic(&self) -> bool { 
        if self.is_empty() {
            return false;
        }

        for statement in self {
            if statement.is_static() {
                return false;
            }
        }

        true
    }
}

impl IsDynamic for IfStatement {
    fn is_dynamic(&self) -> bool {
        self.block.is_dynamic()
            && self.else_if.as_ref().as_ref().map(|else_if| else_if.is_dynamic()).unwrap_or(true)
            && self.else_block.as_ref().map(|block| block.is_dynamic()).unwrap_or(true)
            && self.expr.is_dynamic()
    }
}

impl IsDynamic for Function {
    fn is_dynamic(&self) -> bool { 
        match self {
            Function::Macro    { .. } => true,
            Function::Function { .. } => false
        }
    }
}

impl Generator {
    pub fn simplify_expression(&self, expression: &Expression) -> Expression {
        // TODO
        expression.clone()
    }
}

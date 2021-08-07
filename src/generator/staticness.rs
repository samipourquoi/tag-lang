use crate::parser::function::Function;
use crate::parser::statement::IfStatement;
use crate::parser::statement::Statement;
use crate::generator::Generator;
use crate::parser::expression::{Expression, Term, Summand, VariableName};
use crate::parser::statement::VariableAssignment;

pub trait IsStatic {
    fn is_static(&self) -> bool;
    fn is_dynamic(&self) -> bool { !self.is_static() }
}

impl IsStatic for Expression {
    fn is_static(&self) -> bool {
        match self {
            Expression::Sum(summand, expr) => summand.is_static() && expr.is_static(),
            Expression::Summand(summand) => summand.is_static(),
            Expression::Boolean(_) => true,
            Expression::Variable(var) => var.is_static()
        }
    }
}

impl IsStatic for Summand {
    fn is_static(&self) -> bool {
        match self {
            Summand::Multiplication(term, summand) => term.is_static() && summand.is_static(),
            Summand::Term(term) => term.is_static()
        }
    }
}

impl IsStatic for Term {
    fn is_static(&self) -> bool {
        match self {
            Term::Number(_) => true,
            Term::Expression(expr) => expr.is_static()
        }
    }
}

impl IsStatic for VariableAssignment {
    fn is_static(&self) -> bool {
        self.var.is_static()
    }
}

impl IsStatic for VariableName {
    fn is_static(&self) -> bool {
        match self {
            VariableName::Dynamic(_) => false,
            VariableName::Static(_) => true
        }
    }
}

impl IsStatic for Statement {
    fn is_static(&self) -> bool { 
        match self {
            Statement::IfStatement(if_stmt) => if_stmt.is_static(),
            Statement::Command(_) => true,
            Statement::VariableAssignment(var) => var.is_static(),
            Statement::FunctionDeclaration(function) => function.is_static(),
            _ => false
        }
    }
}

impl IsStatic for Vec<Statement> {
    fn is_static(&self) -> bool { 
        self.iter().all(Statement::is_static)
    }
}

impl IsStatic for IfStatement {
    fn is_static(&self) -> bool {
        self.block.is_static()
            && self.else_if.as_ref().as_ref().map(|else_if| else_if.is_static()).unwrap_or(true)
            && self.else_block.as_ref().map(|block| block.is_static()).unwrap_or(true)
            && self.expr.is_static()
    }
}

impl IsStatic for Function {
    fn is_static(&self) -> bool { 
        match self {
            Function::Macro    { .. } => true,
            Function::Function { .. } => false
        }
    }
}

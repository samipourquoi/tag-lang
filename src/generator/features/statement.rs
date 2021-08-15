use crate::generator::Generator;
use crate::parser::statement::{Statement, VariableAssignment};
use crate::generator::staticness::IsStatic;
use crate::generator::scopes;
use crate::errors::CompilerError;

impl Generator {
    pub fn generate_scoped_statements(&mut self, statements: Vec<Statement>) -> Result<(), CompilerError> {
        if Self::requires_scope(&statements) {
            self.push_scope();
            self.generate_statements(statements)?;
            self.pop_scope();
        } else {
            self.generate_statements(statements)?;
        }

        Ok(())
    }

    pub fn generate_statements(&mut self, statements: Vec<Statement>) -> Result<(), CompilerError> {
        // first we analyze the statements
        // (e.g. we register the functions)

        let functions: Vec<_> = statements.iter().filter_map(|statement| {
            match statement {
                Statement::FunctionDeclaration(func) => Some(func.clone()),
                _ => None
            }
        }).collect();

        for func in &functions {
            if func.is_dynamic() && func.signature.get_static_args().is_empty() {
                let name = self.push_file();
                self.register_function(func.clone(), Some(name));
            } else {
                self.register_function(func.clone(), None);
            }
        }

        for func in functions.iter().rev() {
            if func.is_dynamic() && func.signature.get_static_args().is_empty() {
                self.generate_function(func.clone());
            }
        }

        for statement in statements {
            self.generate_statement(statement)?;
        }

        Ok(())
    }

    pub fn generate_statement(&mut self, statement: Statement) -> Result<(), CompilerError> {
        use Statement::*;

        match statement {
            Command(cmd) => self.generate_command(cmd),
            IfStatement(if_stmt) => self.generate_if_statement(if_stmt),
            VariableAssignment(assignment) => self.generate_variable_assignment(assignment),
            FunctionDeclaration(func) => Ok(()),
            FunctionCall(call) => self.generate_function_call(call),
            _ => todo!()
        }
    }

    pub fn generate_variable_assignment(&mut self, assignment: VariableAssignment) -> Result<(), CompilerError> {
        if assignment.is_dynamic() {
            self.register_runtime_variable(&assignment.signature);
            self.generate_expression(assignment.value)?;
            self.write(format!("data modify storage tag:runtime vars[-1].\"{}\" set from storage tag:runtime stack[-1].@",
                               assignment.signature.name.get_name()));
            self.generate_pop_expression();
        } else {
            self.assign_static_variable(assignment)?;
        }

        Ok(())
    }
}
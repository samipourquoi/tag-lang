use crate::generator::Generator;
use crate::parser::statement::{Statement, VariableAssignment};
use crate::generator::staticness::IsStatic;
use crate::generator::scopes;
use crate::errors::CompilerError;

impl Generator {
    pub fn generate_statements(&mut self, statements: Vec<Statement>) -> Result<(), CompilerError> {
        // first we analyze the statements
        // (e.g. we register the functions)
        for statement in &statements {
            if let Statement::FunctionDeclaration(func) = statement {
                let name = self.generate_function(func.clone())?;
                self.register_function(func.clone(), name);
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
            self.write(format!("data modify storage tag:runtime vars[-1].\"{}\" set from storage tag:runtime stack[-1]",
                               assignment.signature.name.get_name()));
            self.generate_pop_expression();
        } else {
            self.assign_static_variable(assignment)?;
        }

        Ok(())
    }
}
use crate::generator::Generator;
use crate::parser::statement::{Statement, VariableAssignment};
use crate::generator::staticness::IsStatic;
use crate::generator::scopes;

impl Generator {
    pub fn generate_statements(&mut self, statements: Vec<Statement>) {
        self.push_scope();

        // first we analyze the statements
        // (e.g. we register the functions)
        for statement in &statements {
            if let Statement::FunctionDeclaration(func) = statement {
                self.register_function(func.signature.clone())
            }
        }

        for statement in statements {
            self.generate_statement(statement);
        }

        self.pop_scope();
    }

    pub fn generate_statement(&mut self, statement: Statement) {
        use Statement::*;

        match statement {
            Command(cmd) => {
                self.generate_command(cmd);
            },
            IfStatement(if_stmt) => {
                self.generate_if_statement(if_stmt);
            },
            VariableAssignment(assignment) => {
                self.generate_variable_assignment(assignment);
            },
            _ => todo!()
        }
    }

    pub fn generate_variable_assignment(&mut self, assignment: VariableAssignment) {
        if assignment.is_dynamic() {
            self.generate_expression(assignment.value);
            self.write(format!("data modify storage tag:runtime vars[-1].\"{}\" set from storage tag:runtime stack[-1]",
                               assignment.signature.name.get_name()));
            self.generate_pop_expression();
        } else {
            self.assign_static_variable(assignment);
        }
    }
}
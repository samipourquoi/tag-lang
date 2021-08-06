use crate::generator::Generator;
use crate::parser::statement::{Statement, VariableAssignment};
use crate::generator::simplify::IsDynamic;

impl Generator {
    pub fn generate_statements(&mut self, statements: Vec<Statement>) {
        self.push_scope();
        self.write("data modify storage tag:runtime vars append value {}".to_string());

        for statement in statements {
            self.generate_statement(statement);
        }

        self.pop_scope();
        self.write(format!("data remove storage tag:runtime vars[-1]"));
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
            }
        }
    }

    pub fn generate_variable_assignment(&mut self, assignment: VariableAssignment) {
        self.assign_variable(&assignment);

        if assignment.is_dynamic() {
            self.generate_expression(assignment.value);
            self.write(format!("data modify storage tag:runtime vars[-1].\"{}\" set from storage tag:runtime stack[-1]",
                               assignment.var.get_name()));
            self.generate_pop_expression();
        }
    }
}
use crate::generator::Generator;
use crate::parser::Statement;

impl Generator {
    pub(in crate::generator) fn generate_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.generate_statement(statement);
        }
    }

    pub(in crate::generator) fn generate_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Command(cmd) => {
                self.generate_command(cmd);
            },
            Statement::IfStatement(if_stmt) => {
                self.generate_if_statement(if_stmt);
            },
            _ => ()
        }
    }
}
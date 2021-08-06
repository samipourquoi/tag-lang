use crate::generator::Generator;
use crate::parser::statement::Statement;

impl Generator {
    pub fn generate_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.generate_statement(statement);
        }
    }

    pub fn generate_statement(&mut self, statement: Statement) {
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
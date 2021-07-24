use super::parser::{AST};
use crate::parser::{Statement, Command, IfStatement, Expression};
use std::collections::HashMap;

#[derive(Debug)]
struct GenContext {
    file_name_stack: Vec<String>,
    files: HashMap<String, Vec<String>>,
    file_counter: i32
}

impl GenContext {
    fn new() -> Self {
        let mut ctx = GenContext {
            file_name_stack: vec![],
            files: HashMap::new(),
            file_counter: -1
        };
        ctx.push_file();
        ctx
    }

    fn write(&mut self, content: String) {
        let name = self.file_name_stack.last().expect("file name stack is empty");
        let file = self.files.get_mut(name).unwrap();
        file.push(content);
    }

    fn push_file(&mut self) -> String {
        self.file_counter += 1;
        let function_name = self.file_counter.to_string();
        self.files.insert(function_name.clone(), vec![]);
        self.file_name_stack.push(function_name.clone());

        function_name.clone()
    }

    fn pop_file(&mut self) {
        self.file_name_stack.pop();
    }

    fn generate_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.generate_statement(statement);
        }
    }

    fn generate_statement(&mut self, statement: Statement) {
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

    fn generate_command(&mut self, cmd: Command) {
        self.write(cmd.value);
    }

    fn generate_if_statement(&mut self, if_stmt: IfStatement) {
        self.generate_expression(if_stmt.expr);
        let fn_name = self.push_file();
        self.generate_statements(if_stmt.block);
        self.pop_file();

        self.write(format!("execute if data storage tag:runtime stack[-1] run function tag:{}", fn_name));
        self.write(format!("data remove storage tag:runtime stack[-1]"));
    }

    fn generate_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Boolean(bl) => {
                self.write(format!("data modify storage tag:runtime stack append {}", bl));
            },
            _ => ()
        }
    }
}

pub fn generate(ast: AST) {
    let mut ctx = GenContext::new();

    ctx.generate_statements(ast.statements);

    dbg!(ctx);
}

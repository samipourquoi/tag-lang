mod condition;
mod expression;
mod statement;
mod command;

use super::parser::{AST};
use std::collections::HashMap;

#[derive(Debug)]
struct Generator {
    file_name_stack: Vec<String>,
    files: HashMap<String, Vec<String>>,
    file_counter: i32
}

impl Generator {
    fn new() -> Self {
        let mut ctx = Generator {
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


}

pub fn generate(ast: AST) {
    let mut ctx = Generator::new();

    ctx.generate_statements(ast.statements);

    dbg!(ctx);
}
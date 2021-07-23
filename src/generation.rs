use super::parser::{AST};
use crate::parser::{Statement, Command};

#[derive(Debug)]
struct GenContext {
    files: Vec<Vec<String>>
}

impl GenContext {
    fn new() -> Self {
        GenContext {
            files: vec![vec![]]
        }
    }

    fn write(&mut self, content: String) {
        let file = self.files.iter_mut().last().unwrap();
        file.push(content);
    }

    fn push_file(&mut self) {
        self.files.push(vec![]);
    }

    fn pop_file(&mut self) -> Option<Vec<String>> {
        self.files.pop()
    }

    fn generate_command(&mut self, cmd: Command) {
        self.write(cmd.value);
    }
}

pub fn generate(ast: AST) {
    let mut ctx = GenContext::new();

    for statement in ast.statements {
        match statement {
            Statement::Command(cmd) => {
                ctx.generate_command(cmd);
            }
        }
    }

    dbg!(ctx);
}

mod features;
mod scopes;
mod typing;
mod simplify;
pub mod staticness;
pub mod datapack;

use super::parser::{AST};
use std::collections::HashMap;
use crate::errors::CompilerError;
use crate::parser::statement::{VariableAssignment, VariableSignature};
use crate::parser::expression::VariableName;
use crate::parser::typing::Typing;
use crate::CompileOptions;

#[derive(Debug)]
pub struct Generator {
    file_name_stack: Vec<String>,
    files: HashMap<String, Vec<String>>,
    file_counter: i32,
    scopes: Vec<self::scopes::Scope>,
    namespace: String
}

impl Generator {
    fn new(options: &CompileOptions) -> Self {
        let mut ctx = Generator {
            file_name_stack: vec![],
            files: HashMap::new(),
            file_counter: -1,
            scopes: vec![],
            namespace: options.namespace.clone()
        };
        ctx.push_file();
        ctx.push_scope();
        ctx
    }

    fn write<S: ToString>(&mut self, content: S) {
        let name = self.file_name_stack.last().expect("file name stack is empty");
        let file = self.files.get_mut(name).unwrap();
        file.push(content.to_string());
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

    fn generate_push_to_stack<S: std::fmt::Display>(&mut self, value: S) {
        self.write("data modify storage tag:runtime stack append value {}");
        self.write(format!("data modify storage tag:runtime stack[-1].@ set value {}", value));
    }
}

pub fn generate(ast: AST, options: CompileOptions) -> Result<(), CompilerError> {
    let mut ctx = Generator::new(&options);

    ctx.generate_statements(ast.statements)?;
    dbg!(&ctx);
    ctx.write_datapack(options)?;
    Ok(())
}

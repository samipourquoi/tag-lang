use crate::parser::function::Function;
use crate::generator::Generator;

impl Generator {
    pub fn generate_function(&mut self, function: Function) {
        match function {
            Function::Macro {
                name,
                static_args,
                block
            } => {

            },
            Function::Function {
                name,
                static_args,
                dyn_args,
                block
            } => {

            }
        }
    }
}

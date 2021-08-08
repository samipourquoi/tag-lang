use crate::parser::function::Function;
use crate::generator::Generator;
use crate::generator::staticness::IsStatic;

impl Generator {
    pub fn generate_function(&mut self, function: Function) {

        if function.is_dynamic() && function.signature.static_args.is_empty() {
            self.push_file();
            self.push_scope();

            for arg in &function.signature.dyn_args {
                self.register_runtime_variable(arg);
            }

            self.generate_statements(function.block);
            self.pop_scope();
            self.pop_file();
        }
    }
}

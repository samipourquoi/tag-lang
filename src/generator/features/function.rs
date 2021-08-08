use crate::parser::function::Function;
use crate::generator::Generator;

impl Generator {
    pub fn generate_function(&mut self, function: Function) {
        self.push_file();

        self.generate_statements(function.block);

        self.pop_file();
    }
}

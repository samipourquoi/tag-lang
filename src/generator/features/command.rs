use crate::generator::Generator;
use crate::parser::statement::Command;
use crate::generator::simplify::Simplify;

impl Generator {
    pub fn generate_command(&mut self, cmd: Command) {
        let start: Vec<_> = cmd.start.iter() 
            .cloned()
            .map(|(string, expr)| {
                let to_string: String = expr.simplify(&self).unwrap();
                string + to_string.as_str()
            })
            .collect();
        let interpolated = start.join("") + cmd.end.as_str();
        self.write(interpolated);
    }
}

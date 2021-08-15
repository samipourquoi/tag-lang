use crate::generator::Generator;
use crate::parser::statement::Command;
use crate::generator::simplify::Simplify;
use crate::errors::CompilerError;

impl Generator {
    pub fn generate_command(&mut self, cmd: Command) -> Result<(), CompilerError> {
        let mut start: Vec<String> = vec![];
        for (string, expr) in cmd.start.clone() {
            let to_string: String = expr.to_string(&self)?;
            start.push(string + to_string.as_str());
        }

        let interpolated = start.join("") + cmd.end.as_str();
        self.write(interpolated);

        Ok(())
    }
}

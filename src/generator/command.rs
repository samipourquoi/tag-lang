use crate::generator::Generator;
use crate::parser::Command;

impl Generator {
    pub fn generate_command(&mut self, cmd: Command) {
        self.write(cmd.value);
    }
}

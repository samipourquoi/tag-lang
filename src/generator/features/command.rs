use crate::generator::Generator;
use crate::parser::statement::Command;

impl Generator {
    pub fn generate_command(&mut self, cmd: Command) {
        dbg!(cmd);
    }
}

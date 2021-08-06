use crate::generator::Generator;
use crate::parser::IfStatement;

impl Generator {
    pub(in crate::generator) fn generate_if_statement(&mut self, if_stmt: IfStatement) {
        self.generate_expression(if_stmt.expr);

        let fn_name = self.push_file();
        self.generate_statements(if_stmt.block);
        self.pop_file();

        self.write(format!("execute if data storage tag:runtime stack[-1] run function tag:{}", fn_name));

        if let Some(else_if) = *if_stmt.else_if {
            let name = self.push_file();
            self.generate_if_statement(else_if);
            self.pop_file();
            self.write(format!("execute unless data storage tag:runtime stack[-1] run function tag:{}", name));
        } else if let Some(else_block) = if_stmt.else_block {
            let name = self.push_file();
            self.generate_statements(else_block);
            self.pop_file();
            self.write(format!("execute unless data storage tag:runtime stack[-1] run function tag:{}", name));
        }

        self.write(format!("data remove storage tag:runtime stack[-1]"));
    }

}

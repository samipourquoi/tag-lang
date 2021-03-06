use crate::generator::Generator;
use crate::parser::statement::IfStatement;
use crate::errors::CompilerError;

impl Generator {
    pub fn generate_if_statement(&mut self, if_stmt: IfStatement) -> Result<(), CompilerError> {
        self.generate_expression(if_stmt.expr)?;

        self.write("data modify storage tag:runtime condition set from storage tag:runtime stack[-1].@");

        let fn_name = self.push_file();
        self.generate_scoped_statements(if_stmt.block)?;
        self.pop_file();

        self.write(format!("execute if data storage tag:runtime {{condition:true}} run function {}:{}", self.namespace, fn_name));

        if let Some(else_if) = *if_stmt.else_if {
            let name = self.push_file();
            self.generate_if_statement(else_if)?;
            self.pop_file();
            self.write(format!("execute unless data storage tag:runtime {{condition:true}} run function {}:{}", self.namespace, name));
        } else if let Some(else_block) = if_stmt.else_block {
            let name = self.push_file();
            self.generate_scoped_statements(else_block)?;
            self.pop_file();
            self.write(format!("execute unless data storage tag:runtime {{condition:true}} run function {}:{}", self.namespace, name));
        }

        self.generate_pop_expression();

        Ok(())
    }
}

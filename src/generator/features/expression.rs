use crate::generator::Generator;
use crate::parser::expression::Expression;

impl Generator {
    pub fn generate_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Boolean(bl) => {
                self.write(format!("data modify storage tag:runtime stack append value {}", bl));
            },
            Expression::Variable(var) => {
                let path = self.get_variable_nbt_path(&var);
                self.write(format!("data modify storage tag:runtime stack append from storage tag:runtime {}", path))
            },
            _ => todo!()
        }
    }

    pub fn generate_pop_expression(&mut self) {
        self.write(format!("data remove storage tag:runtime stack[-1]"));
    }
}

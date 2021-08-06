use crate::generator::Generator;
use crate::parser::expression::Expression;

impl Generator {
    pub(in crate::generator) fn generate_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Boolean(bl) => {
                self.write(format!("data modify storage tag:runtime stack append {}", bl));
            },
            _ => ()
        }
    }
}

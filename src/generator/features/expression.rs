use crate::parser::expression::Term;
use crate::parser::expression::Summand;
use crate::generator::Generator;
use crate::parser::expression::Expression;
use crate::generator::staticness::IsStatic;
use crate::generator::simplify::Simplify;
use crate::errors::CompilerError;

impl Generator {
    pub fn generate_expression(&mut self, expr: Expression) -> Result<(), CompilerError> {
        if expr.is_static() {
            let value: String = expr.to_string(self)?;
            self.generate_push_to_stack(value);
            return Ok(());
        }

        match expr {
            Expression::Boolean(bl, _) => {
                self.generate_push_to_stack(bl);
            },
            Expression::Sum(summand, expr, _) => {
                self.generate_summand(summand)?;
                self.generate_expression(*expr)?;
                self.write("execute store result score %a __tag__ run data get storage tag:runtime stack[-1].@");
                self.write("execute store result score %b __tag__ run data get storage tag:runtime stack[-2].@");

                self.generate_pop_expression();

                self.write("execute store result storage tag:runtime stack[-1].@ int 1 run scoreboard players operation %a __tag__ *= %b __tag__");
            },
            Expression::Summand(summand, _) => self.generate_summand(summand)?,
            _ => todo!()
        };

        Ok(())
    }

    pub fn generate_summand(&mut self, summand: Summand) -> Result<(), CompilerError> {
        match summand {
            Summand::Multiplication(term, summand) => {
                self.generate_term(term)?;
                self.generate_summand(*summand)?;
                self.write("execute result store score %a __tag__ run data get storage tag:runtime stack[-1].@");
                self.write("execute result store score %b __tag__ run data get storage tag:runtime stack[-2].@");

                self.generate_pop_expression();

                self.write("execute store result storage tag:runtime stack[-1].@ int 1 run scoreboard players operation %a __tag__ *= %b __tag__");

                Ok(())
            },
            Summand::Term(term) => self.generate_term(term)
        }
    }

    pub fn generate_term(&mut self, term: Term) -> Result<(), CompilerError> {
        match term {
            Term::Number(n) => self.generate_push_to_stack(n),
            Term::Expression(expr) => self.generate_expression(*expr)?,
            Term::FunctionCall(_call) => unimplemented!(),
            Term::Variable(var) => {
                // if var.is_static() {
                //     let value = Expression::Summand(
                //         Summand::Term(
                //             Term::Variable(var.clone())
                //         )
                //     );
                //     self.write(format!("data modify storage tag:runtime stack append value {}",
                //         <Expression as Simplify<String>>::simplify(&value, self).unwrap()));
                // } else {
                    let path = self.get_variable_nbt_path(&var);
                    self.write("data modify storage tag:runtime stack append value {}");
                    self.write(format!("data modify storage tag:runtime stack[-1].@ set from storage tag:runtime {}", path))
                // }
            },
            Term::String(str) => self.generate_push_to_stack(str)
        };

        Ok(())
    }

    pub fn generate_pop_expression(&mut self) {
        self.write(format!("data remove storage tag:runtime stack[-1]"));
    }
}

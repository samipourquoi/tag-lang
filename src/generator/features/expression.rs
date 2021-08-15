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
            let value: String = expr.simplify(self).unwrap();
            self.write(format!("data modify storage tag:runtime stack append value {}", value));
            return Ok(());
        }

        match expr {
            Expression::Boolean(bl, _) => {
                self.write(format!("data modify storage tag:runtime stack append value {}", bl));
            },
            Expression::Sum(summand, expr, _) => {
                self.generate_summand(summand)?;
                self.generate_expression(*expr)?;
                self.write("execute store score %a __tag__ run data get storage tag:runtime stack[-1]");
                self.write("execute store score %b __tag__ run data get storage tag:runtime stack[-2]");

                self.generate_pop_expression();

                self.write("scoreboard players operation %a __tag__ += %b __tag__");
                self.write("execute store storage tag:runtime stack[-1] int run scoreboard players get %a __tag__");
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
                self.write("execute store score %a __tag__ run data get storage tag:runtime stack[-1]");
                self.write("execute store score %b __tag__ run data get storage tag:runtime stack[-2]");

                self.generate_pop_expression();

                self.write("scoreboard players operation %a __tag__ *= %b __tag__");
                self.write("execute store storage tag:runtime stack[-1] int run scoreboard players get %a __tag__");

                Ok(())
            },
            Summand::Term(term) => self.generate_term(term)
        }
    }

    pub fn generate_term(&mut self, term: Term) -> Result<(), CompilerError> {
        match term {
            Term::Number(n) => 
                self.write(format!("data modify storage tag:runtime stack append value {}", n)),
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
                    self.write(format!("data modify storage tag:runtime stack append from storage tag:runtime {}", path))
                // }
            }
        };

        Ok(())
    }

    pub fn generate_pop_expression(&mut self) {
        self.write(format!("data remove storage tag:runtime stack[-1]"));
    }
}

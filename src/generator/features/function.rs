use crate::parser::statement::VariableSignature;
use crate::parser::statement::VariableAssignment;
use crate::parser::function::FunctionSignature;
use crate::parser::function::FunctionCall;
use crate::parser::function::Function;
use crate::generator::Generator;
use crate::generator::staticness::IsStatic;

impl Generator {
    pub fn generate_function(&mut self, function: Function) -> Option<String> {
        if function.is_dynamic() && function.signature.get_static_args().is_empty() {
            let name = self.push_file();
            self.push_scope();

            for sign in &function.signature.get_dynamic_args() {
                self.register_runtime_variable(sign);
                self.write(format!(
                    "data modify storage tag:runtime vars[-1].\"{}\" append value from storage tag:runtime stack[-1]",
                    sign.name.get_name()
                ));
                self.generate_pop_expression();
            }

            self.generate_statements(function.block);
            self.pop_scope();
            self.pop_file();

            return Some(name);
        }

        None
    }

    pub fn generate_function_call(&mut self, function_call: FunctionCall) {
        if let Some(info) = self.resolve_function_call(&function_call) {
            let (func, file_name) = info.clone();
            let args: Vec<_> = func.signature.args.iter().zip(function_call.args).collect();
            let static_args: Vec<_> = args.iter().filter(|(sign, _)| sign.is_static()).collect();
            let dyn_args: Vec<_> = args.iter().filter(|(sign, _)| sign.is_dynamic()).collect();
            
            for (sign, expr) in &static_args {
                self.assign_static_variable(VariableAssignment {
                    signature: sign.clone().clone(),
                    value: expr.clone()
                });
            }

            if func.is_static() {
                self.push_static_scope();
                self.generate_statements(func.block.clone());
                self.pop_static_scope();
            } else if func.is_dynamic() && !static_args.is_empty() {
                for (_, expr) in &dyn_args {
                    self.generate_expression(expr.clone());
                }

                self.push_scope();

                for (sign, _) in &dyn_args {
                    self.register_runtime_variable(sign);
                    self.write(format!(
                        "data modify storage tag:runtime vars[-1].\"{}\" append value from storage tag:runtime stack[-1]",
                        sign.name.get_name()
                    ));
                    self.generate_pop_expression();
                }

                let name = self.push_file();

                self.generate_statements(func.block.clone());
                self.pop_scope();
                self.pop_file();

                self.write(format!("function tag:{}", name))
            } else if func.is_dynamic() && static_args.is_empty() {
                for (_, expr) in &dyn_args {
                    self.generate_expression(expr.clone());
                }
                self.write(format!("function tag:{}", file_name.unwrap()))
            }
        } else {
            panic!("can't resolve function call");
        }
    }
}

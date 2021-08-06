use crate::parser::expression::{VariableName, Expression};
use crate::generator::Generator;
use crate::parser::statement::VariableAssignment;
use std::collections::HashMap;
use std::any::Any;
use crate::generator::simplify::IsDynamic;
use crate::parser::typing::Typing;

#[derive(Debug)]
pub(in crate::generator) struct Scope {
    runtime_variables: HashMap<VariableName, Typing>,
    comptime_variables: HashMap<VariableName, Expression>
}

impl Generator {
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {
            runtime_variables: HashMap::new(),
            comptime_variables: HashMap::new()
        });
    }

    pub fn pop_scope(&mut self) {
        let scope = self.scopes.pop().expect("can't pop a scope if there is none left.");
    }

    pub fn assign_variable(&mut self, declaration: &VariableAssignment) {
        match declaration.var {
            VariableName::Dynamic(_) => {
                let scope = self.scopes.last_mut()
                    .expect("can't assign a variable if there is no scope left.");

                scope.runtime_variables.insert(declaration.var.clone(), declaration.typing.clone());
            }
            VariableName::Static(_) if declaration.value.is_static() => {
                let expr = self.simplify_expression(&declaration.value);
                let scope = self.scopes.last_mut()
                    .expect("can't assign a variable if there is no scope left.");

                scope.comptime_variables.insert(declaration.var.clone(), expr);
            },
            _ => panic!("trying to use a dynamic value in a static context")
        };
    }
}

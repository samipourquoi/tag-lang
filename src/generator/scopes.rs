use crate::parser::statement::VariableSignature;
use std::collections::HashSet;
use crate::parser::function::FunctionSignature;
use crate::parser::function::Function;
use crate::parser::expression::{VariableName, Expression};
use crate::generator::Generator;
use crate::parser::statement::VariableAssignment;
use std::collections::HashMap;
use crate::generator::staticness::IsStatic;
use crate::parser::typing::Typing;
use crate::generator::simplify::Simplify;

#[derive(Debug)]
pub(in super) struct Scope {
    runtime_variables: HashMap<VariableName, Typing>,
    comptime_variables: HashMap<VariableName, Expression>,
    functions: HashSet<FunctionSignature>
}

impl Generator {
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {
            runtime_variables: HashMap::new(),
            comptime_variables: HashMap::new(),
            functions: HashSet::new()
        });
        self.write("data modify storage tag:runtime vars append value {}");
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("can't pop a scope if there is none left.");
        self.write("data remove storage tag:runtime vars[-1]");
    }

    pub(in super) fn peek_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("no scope")
    }

    pub fn register_runtime_variable(&mut self, signature: &VariableSignature) {
        assert!(signature.name.is_dynamic());
        let scope = self.peek_scope();
        scope.runtime_variables.insert(signature.name.clone(), signature.typing.clone());
    }

    pub fn assign_static_variable(&mut self, assignment: VariableAssignment) {
        assert!(assignment.signature.name.is_static() && assignment.value.is_static());

        // we'll have to resolve static variables here, but we need to do
        // type inference and other things first.
        // it'll just crash if we try to do something like `a := a + 1` for now.
        // let expr = assignment.value.simplify(self).expect("can't resolve static variable");

        let scope = self.peek_scope();
        scope.comptime_variables.insert(assignment.signature.name, assignment.value);
    }

    pub fn get_variable_nbt_path(&self, var: &VariableName) -> String {
        let index = -1 - self.scopes.iter()
            .rev()
            .position(|scope| scope.runtime_variables.contains_key(var))
            .unwrap() as i32;
        format!("vars[{}].\"{}\"", index, var.get_name())
    }

    pub fn get_static_variable_value(&self, var: &VariableName) -> Option<Expression> {
        let scope = self.scopes.iter()
            .rev()
            .find(|scope| scope.comptime_variables.contains_key(var));
        scope.map(|scope| scope.comptime_variables.get(var).unwrap().clone())
    }

    pub fn register_function(&mut self, signature: FunctionSignature) {
        let scope = self.peek_scope();
        scope.functions.insert(signature);
    }

    pub fn does_function_exist(&self, signature: FunctionSignature) -> bool {
        self.scopes.iter()
            .rev()
            .find(|scope| scope.functions.contains(&signature))
            .is_some()
    }
}

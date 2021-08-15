use crate::parser::function::FunctionCall;
use crate::parser::statement::{Statement, IfStatement};
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
use std::cmp::Ordering;
use crate::errors::CompilerError;

#[derive(Debug)]
pub(in super) struct Scope {
    runtime_variables: HashMap<VariableName, Typing>,
    comptime_variables: HashMap<VariableName, Expression>,
    functions: HashMap<FunctionSignature, (Function, Option<String>)>
}

impl Generator {
    pub fn push_static_scope(&mut self) {
        self.scopes.push(Scope {
            runtime_variables: HashMap::new(),
            comptime_variables: HashMap::new(),
            functions: HashMap::new()
        });
    }

    pub fn pop_static_scope(&mut self) {
        self.scopes.pop().expect("can't pop a scope if there is none left.");
    }

    pub fn push_scope(&mut self) {
        self.push_static_scope();
        self.write("data modify storage tag:runtime vars append value {}");
    }

    pub fn pop_scope(&mut self) {
        self.pop_static_scope();
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

    pub fn assign_static_variable(&mut self, assignment: VariableAssignment) -> Result<(), CompilerError> {
        if assignment.signature.name.is_static() && assignment.value.is_dynamic() {
            return Err(CompilerError::from((assignment.position, "can't assign a dynamic value to a static variable")));
        }

        // we'll have to resolve static variables here, but we need to do
        // type inference and other things first.
        // it'll just crash if we try to do something like `a := a + 1` for now.
        // let expr = assignment.value.simplify(self).expect("can't resolve static variable");

        let scope = self.peek_scope();
        scope.comptime_variables.insert(assignment.signature.name, assignment.value);

        Ok(())
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

    pub fn register_function(&mut self, function: Function, file_name: Option<String>) {
        let scope = self.peek_scope();
        scope.functions.insert(function.signature.clone(), (function, file_name));
    }

    pub fn resolve_function_call(&self, call: &FunctionCall) -> Option<&(Function, Option<String>)> {
        // To resolve a function, we check if the call signature is the same
        // as the function signature.
        // However, compile-time (=static) variables can also be used as dynamic
        // variables, and we want to support overloading. So for each function,
        // we associate a "score". A compile-time value matching with a dynamic
        // argument won't add to the score, while a compile-time value matching
        // with a static argument will.
        // TODO: other factors will be type matching (i.e. the stricter the type is,
        // the more important it is).

        type Info = (Function, Option<String>);

        let candidates: Vec<(&Info, i32)> = self.scopes.iter().rev().map(|scope| {
            scope.functions.iter().filter_map(|(sign, info)| {
                let mut score = 0;

                if info.0.signature.name != call.name {
                    return None;
                }

                for (sign_arg, call_arg) in sign.args.iter().zip(&call.args) {
                    if sign_arg.is_static() && call_arg.is_dynamic() {
                        return None;
                    }

                    // TODO: check type

                    if sign_arg.is_static() && call_arg.is_static() {
                        score += 1;
                    }
                }

                Some((info, score))
            }).collect::<Vec<(&Info, i32)>>()
        }).flatten().collect::<Vec<(&Info, i32)>>();

        candidates.iter().max_by(|(_, score1), (_, score2)| score1.cmp(score2))
            .map(|candidate| candidate.0)
    }

    pub fn requires_scope(statements: &Vec<Statement>) -> bool {
        fn if_statement_requires_scope(r#if: &IfStatement) -> bool {
            Generator::requires_scope(&r#if.block)
                || Generator::requires_scope(r#if.else_block.as_ref().unwrap_or(&vec![]))
                || (*r#if.else_if).as_ref().map_or(false, |r#if| if_statement_requires_scope(r#if))
        }

        statements.iter().any(|statement| match statement {
            Statement::IfStatement(r#if) => if_statement_requires_scope(r#if),
            Statement::VariableAssignment(_) => true,
            Statement::FunctionDeclaration(_) => true,
            _ => false
        })
    }
}

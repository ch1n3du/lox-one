use std::collections::HashMap;

use crate::{
    ast::{Expr, Stmt},
    lox_value::LoxValue,
};

use super::{
    error::{RuntimeError, RuntimeResult},
    Interpreter,
};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> RuntimeResult<()> {
        use Stmt::*;

        match stmt {
            Block(statements) => {
                self.begin_scope();
                self.resolve_stmts(statements)?;
                self.end_scope();
            }
            Stmt::Var {
                name,
                initializer,
                postion: _,
            } => {
                // Declare the variable in the innnermost scope marking it as 'still resolving'.
                self.declare(name);
                match initializer {
                    Expr::Value { value, position: _ } => {
                        // If the variable has a non-null initializer resolve the initializer.
                        if !value.is_nil() {
                            self.resolve_expr(&initializer)?;
                        }
                    }
                    _ => (),
                }
                self.define(name);
            }
            _ => todo!(),
        }

        Ok(())
    }

    /// TODO Resolves a mutable slice of statements
    fn resolve_stmts(&mut self, stmts: &[Stmt]) -> RuntimeResult<()> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> RuntimeResult<()> {
        match expr {
            Expr::Identifier(name, position) => {
                // Check if variable is initialized yet.
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes.last_mut().unwrap().get(name) {
                        return Err(RuntimeError::VarUsedInOwnInitializer(
                            name.to_owned(),
                            position.to_owned(),
                        ));
                    }
                }

                self.resolve_local(expr, name)?;
            }
            _ => todo!(),
        }

        Ok(())
    }

    // TODO
    fn resolve_local(&mut self, expr: &Expr, name: &str) -> RuntimeResult<()> {
        for (index, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                // self.interpreter.resolve(name, index);

                // return;
            }
        }
        todo!()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Adds the variable to the innermost scope so that it shadows any outer one and so that
    /// we know that the variable exists. We mark it as 'not ready yet' by binding a `false` value to
    /// the scope map. The value associated with a key in the scope map represents whether or now we have
    /// finished resolving the  variable's initializer.
    fn declare(&mut self, name: &str) {
        // If there is no current scope then just return
        if let Some(innermost_scope) = self.scopes.last_mut() {
            innermost_scope.insert(name.to_owned(), false);
        }
    }

    /// After declaring the variable, we resolve it's initializer expressions in that same scope where
    /// the new variable now exists but is unavailable. Once the initializer expression is done, the
    /// variable is 'declared' (it's value in the scope map is set to `true`).
    fn define(&mut self, name: &str) {
        if let Some(innermost_scope) = self.scopes.last_mut() {
            innermost_scope.insert(name.to_owned(), true);
        }
    }
}

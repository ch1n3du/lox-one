use std::collections::HashMap;

use crate::{
    ast::{Expr, Stmt},
    function::FunDecl,
};

use super::{
    error::{RuntimeError, RuntimeResult},
    Interpreter,
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver<'a> {
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
            Var {
                name, initializer, ..
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
            FunStmt {
                fun_declaration: fun_decl @ FunDecl { name, .. },
                ..
            } => {
                self.declare(&name);
                self.define(&name);
                self.resolve_function(fun_decl)?;
            }
            ExprStmt(expr) => {
                self.resolve_expr(expr)?;
            }
            IfStmt {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.resolve_expr(condition)?;
                self.resolve_block(then_branch)?;

                if let Some(else_block) = else_branch {
                    self.resolve_block(&else_block)?;
                }
            }
            PrintStmt(expr) => self.resolve_expr(expr)?,
            ReturnStmt { expr, .. } => {
                if let Some(value) = expr {
                    self.resolve_expr(value)?;
                }
            }
            WhileStmt {
                condition, body, ..
            } => {
                self.resolve_expr(condition)?;
                self.resolve_block(&body)?;
            }
            ContinueStmt(_) => (),
            BreakStmt(_) => (),
        }

        Ok(())
    }

    fn resolve_function(&mut self, fun_declaration: &FunDecl) -> RuntimeResult<()> {
        self.begin_scope();
        for param in &fun_declaration.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_block(&fun_declaration.body)?;
        self.end_scope();

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> RuntimeResult<()> {
        use Expr::*;

        match expr {
            Identifier(name, position) => {
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
            assign_expr @ Assignment { name, value, .. } => {
                self.resolve_expr(&value)?;
                self.resolve_local(assign_expr, name)?;
            }
            Binary { lhs, rhs, .. } => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
            }
            Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee)?;

                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
            }
            Grouping(expr, ..) => {
                self.resolve_expr(expr)?;
            }
            Value { .. } => (),
            Unary { rhs, .. } => self.resolve_expr(rhs)?,
            Ternary {
                condition,
                result_1,
                result_2,
                ..
            } => {
                self.resolve_expr(condition)?;
                self.resolve_expr(result_1)?;
                self.resolve_expr(result_2)?;
            }
        }

        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &str) -> RuntimeResult<()> {
        for (index, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                let depth = self.scopes.len() - 1 - index;
                self.interpreter.resolve(expr, depth)?;
            }
        }
        Ok(())
    }

    /// TODO Resolves a mutable slice of statements
    pub fn resolve_stmts(&mut self, stmts: &[Stmt]) -> RuntimeResult<()> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn resolve_program(&mut self, stmts: &[Stmt]) -> RuntimeResult<()> {
        self.begin_scope();
        self.resolve_stmts(stmts)?;
        self.end_scope();

        Ok(())
    }

    fn resolve_block(&mut self, block: &Stmt) -> RuntimeResult<()> {
        self.begin_scope();
        if let Stmt::Block(stmts) = block {
            self.resolve_stmts(stmts)?;
        }
        self.end_scope();

        Ok(())
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

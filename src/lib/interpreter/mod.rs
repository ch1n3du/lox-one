pub mod environment;
pub mod error;

mod globals;
mod resolver;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::ast::{Expr, Stmt};
use crate::error::{LoxError, LoxResult};
use crate::function::Function;
use crate::interpreter::resolver::Resolver;
use crate::lox_value::LoxValue;
use crate::parser::Parser;
use crate::token::Position;
use crate::token_type::TokenType;

use self::environment::Environment;
use self::error::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct Interpreter {
    pub environment: Environment,
    pub globals: Environment,
    pub locals: HashMap<Position, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Environment::new();

        globals.define(
            "clock",
            LoxValue::Function(Function::new_native_fun(
                "clock".to_string(),
                0,
                globals::clock,
            )),
        );
        Interpreter {
            environment: Environment::new(),
            globals,
            locals: HashMap::new(),
        }
    }

    /// Evaluates an expression.
    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<LoxValue> {
        use Expr::*;
        use LoxValue::*;
        use TokenType::*;

        match expr {
            Value { value, position: _ } => Ok(value.to_owned()),
            Grouping(inner_expr, _position) => self.evaluate(inner_expr),
            Expr::Identifier(name, position) => {
                let res = if let Some(depth) = self.locals.get(position) {
                    if let Some(e) = self.environment.get_at(name, depth.to_owned()) {
                        return Ok(e);
                    } else {
                        Err(RuntimeError::VarDoesNotExist {
                            name: name.to_owned(),
                            position: position.to_owned(),
                        })
                    }
                } else {
                    if let Some(e) = self.globals.get(name) {
                        return Ok(e);
                    } else {
                        Err(RuntimeError::VarDoesNotExist {
                            name: name.to_owned(),
                            position: position.to_owned(),
                        })
                    }
                };
                res
            }
            Assignment {
                name,
                value,
                position,
            } => {
                let previous = self.environment.get(&name);

                if previous.is_none() {
                    Err(RuntimeError::VarDoesNotExist {
                        name: name.to_owned(),
                        position: position.to_owned(),
                    })
                } else {
                    let value = self.evaluate(value.as_ref())?;
                    self.environment.assign(name.as_str(), value);

                    Ok(LoxValue::Nil)
                }
            }
            Unary {
                op,
                rhs,
                position: _,
            } => match (&op.token_type, self.evaluate(rhs.as_ref())?) {
                (Minus, LoxValue::Number(n)) => Ok(LoxValue::Number(-n)),
                (Minus, _) => Err(RuntimeError::Generic(
                    "Expected a number.".to_string(),
                    op.position,
                )),

                (Bang, Boolean(b)) => Ok(LoxValue::Boolean(!b)),
                (Bang, _) => Err(RuntimeError::Generic(
                    "Expected a boolean expression".to_string(),
                    op.position,
                )),

                _ => Err(RuntimeError::Generic(
                    "Expected a unary expression.".to_string(),
                    op.position,
                )),
            },
            Binary {
                lhs,
                op,
                rhs,
                position: _,
            } => {
                let (lhs, rhs) = (self.evaluate(lhs)?, self.evaluate(rhs)?);

                match (&op.token_type, lhs, rhs) {
                    // Arithmetic Operators
                    (Plus, LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                    (Minus, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Number(l - r))
                    }
                    (Star, LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l * r)),
                    (Slash, LoxValue::Number(l), LoxValue::Number(r)) => {
                        if r == 0.0 {
                            return Err(RuntimeError::DivisionByZero(op.position));
                        }
                        Ok(LoxValue::Number(l / r))
                    }

                    // Logical Operators
                    (And, left, right) => {
                        Ok(LoxValue::Boolean(left.is_truthy() && right.is_truthy()))
                    }
                    (Or, left, right) => {
                        Ok(LoxValue::Boolean(left.is_truthy() || right.is_truthy()))
                    }

                    // Comparison Operators
                    (EqualEqual, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l == r))
                    }
                    (BangEqual, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l != r))
                    }
                    (Greater, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l > r))
                    }
                    (GreaterEqual, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l >= r))
                    }
                    (Less, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l < r))
                    }
                    (LessEqual, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Boolean(l <= r))
                    }

                    // String Concatenation
                    (Plus, LoxValue::String(s1), rhs) => Ok(LoxValue::String(format!(
                        "{}{}",
                        s1.to_string(),
                        rhs.to_string()
                    ))),
                    (Star, LoxValue::String(s1), LoxValue::Number(n)) => {
                        Ok(LoxValue::String(s1.repeat(n as usize)))
                    }

                    (_op, _lhs, _rhs) => Err(RuntimeError::Generic(
                        format!("Don't really know: LHS: {} OP: {} RHS: {}", _lhs, _op, _rhs),
                        op.position,
                    )),
                }
            }
            Ternary {
                condition,
                result_1,
                result_2,
                position: _,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.evaluate(result_1)
                } else {
                    self.evaluate(result_2)
                }
            }
            Call {
                callee,
                arguments,
                position,
            } => {
                let callee = self.evaluate(callee)?;

                if let Some(callable) = callee.as_callable() {
                    if callable.arity() == arguments.len() {
                        let mut evaluated_arguments = Vec::new();

                        for argument in arguments {
                            evaluated_arguments.push(self.evaluate(argument)?)
                        }

                        callable.call(self, &evaluated_arguments)
                    } else {
                        Err(RuntimeError::IncorrectArity {
                            name: callable.name(),
                            position: position.to_owned(),
                        })
                    }
                } else {
                    Err(RuntimeError::NotCallable {
                        type_name: callee,
                        position: position.to_owned(),
                    })
                }
            }
        }
    }

    /// Executes a statement.
    pub fn execute(
        &mut self,
        statement: &Stmt,
        in_loop: bool,
        in_function: bool,
    ) -> RuntimeResult<Option<LoxValue>> {
        use Stmt::*;

        match statement {
            ExprStmt(expr) => {
                self.evaluate(expr)?;
            }
            PrintStmt(expr) => {
                println!("{}", self.evaluate(expr)?);
            }
            Var {
                name,
                initializer,
                postion: _,
            } => {
                let initializer = self.evaluate(initializer)?;
                self.environment.define(name, initializer);
            }
            Block(declarations) => {
                self.environment.begin_scope();
                let res = self.interpret(declarations, in_loop, in_function)?;
                self.environment.end_scope();

                return Ok(res);
            }
            IfStmt {
                condition,
                then_branch,
                else_branch,
                position: _,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(&then_branch, true, in_function)?;
                } else if let Some(stmt) = else_branch {
                    self.execute(&stmt, true, in_function)?;
                }
            }
            WhileStmt {
                condition,
                body,
                position: _,
            } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body, true, in_function)?;
                }
            }
            BreakStmt(position) => {
                if in_loop {
                    return Err(RuntimeError::ValidBreak);
                } else {
                    return Err(RuntimeError::InvalidBreak(position.to_owned()));
                }
            }
            ContinueStmt(position) => {
                if in_loop {
                    return Err(RuntimeError::ValidContinue);
                } else {
                    return Err(RuntimeError::InvalidBreak(position.to_owned()));
                }
            }
            FunStmt {
                fun_declaration: decl,
                position: _,
            } => self.environment.define(
                &decl.name,
                LoxValue::Function(Function::User(decl.to_owned())),
            ),
            ReturnStmt { expr, position } => {
                if in_function {
                    if let Some(value) = expr {
                        let value = self.evaluate(value)?;
                        return Ok(Some(value));
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Err(RuntimeError::InvalidReturn(position.to_owned()));
                }
            }
        }

        Ok(None)
    }

    /// Executes statements given.
    pub fn interpret(
        &mut self,
        statements: &[Stmt],
        in_loop: bool,
        in_function: bool,
    ) -> RuntimeResult<Option<LoxValue>> {
        use RuntimeError::*;

        if self.locals.is_empty() {
            let mut resolver = Resolver::new(self);
            resolver.resolve_program(statements)?;
        }

        for statement in statements {
            match self.execute(statement, in_loop, in_function) {
                Ok(None) => {
                    continue;
                }
                Ok(Some(value)) => return Ok(Some(value)),
                Err(ValidBreak) => {
                    return Ok(None);
                }
                Err(ValidContinue) => {
                    continue;
                }
                err => {
                    err?;
                }
            }
        }

        Ok(None)
    }

    fn resolve(&mut self, expr: &Expr, depth: usize) -> RuntimeResult<()> {
        self.locals.insert(expr.get_position().to_owned(), depth);
        Ok(())
    }

    pub fn interpret_str(&mut self, source: &str) -> LoxResult<Option<LoxValue>> {
        let stmts = Parser::parse_str(source)?;

        self.interpret(&stmts, false, false)
            .map_err(|e| LoxError::Runtime(e))
    }
}

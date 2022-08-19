pub mod environment;
pub mod error;
mod globals;
#[cfg(test)]
mod tests;

use crate::ast::{Expr, Stmt};
use crate::function::Function;
use crate::lox_value::LoxValue;
use crate::token_type::TokenType;

use self::environment::Environment;
use self::error::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut environment = Environment::new();

        environment.values.insert(
            "clock".to_string(),
            LoxValue::Function(Function::new_native_fun(
                "clock".to_string(),
                0,
                globals::clock,
            )),
        );
        Interpreter { environment }
    }

    /// Evaluates an expression.
    pub fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        use Expr::*;
        use LoxValue::*;
        use TokenType::*;

        match expr {
            Value { value, position: _ } => Ok(value.to_owned()),
            Grouping(inner_expr, _position) => self.evaluate(inner_expr),
            Expr::Identifier(name, position) => {
                let value = self.environment.get(&name);

                if value.is_some() {
                    Ok(value.unwrap().to_owned())
                } else {
                    Err(RuntimeError::VarDoesNotExist {
                        name: name.to_owned(),
                        position: position.to_owned(),
                    })
                }
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

                // println!(
                //     "Evaluating binary expression: \nLHS: {}\nOP: {}\nRHS: {}",
                //     &lhs, &op, rhs,
                // );

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
                    (Plus, LoxValue::String(s1), LoxValue::String(s2)) => {
                        Ok(LoxValue::String(s1 + s2.as_str()))
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
                postion: _,
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
                postion,
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
                            position: postion.to_owned(),
                        })
                    }
                } else {
                    Err(RuntimeError::NotCallable {
                        type_name: callee,
                        position: postion.to_owned(),
                    })
                }
            }
        }
    }

    pub fn execute_block(
        &mut self,
        declarations: &Vec<Stmt>,
        env: Environment,
        in_loop: bool,
        in_function: bool,
    ) -> RuntimeResult<Option<LoxValue>> {
        let mut env = env;
        env.enclosing = Some(Box::new(self.environment.clone()));
        self.environment = env;
        let res = self.interpret(declarations, in_loop, in_function);
        self.environment = *self.environment.enclosing.clone().unwrap();

        res
    }

    /// Executes a statement.
    fn execute(
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
                // println!("Raw: {}", expr);
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
                let env = Environment::new();
                return self.execute_block(declarations, env, in_loop, in_function);
            }
            IfStmt {
                condition,
                true_stmt,
                false_stmt,
                position: _,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(&true_stmt, true, in_function)?;
                } else if let Some(stmt) = false_stmt {
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
                        return Ok(Some(self.evaluate(value)?));
                    } else {
                        return Err(RuntimeError::InvalidBreak(position.to_owned()));
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
        statements: &Vec<Stmt>,
        in_loop: bool,
        in_function: bool,
    ) -> RuntimeResult<Option<LoxValue>> {
        use RuntimeError::*;

        for statement in statements {
            match self.execute(statement, in_loop, in_function) {
                Ok(None) => {
                    continue;
                }
                Ok(Some(value)) => return Ok(Some(value)),
                Err(ValidBreak) => {
                    println!("About to call 'break': {:?}", self);
                    return Ok(None);
                }
                Err(ValidContinue) => {
                    println!("About to call 'continue': {:?}", self);
                    continue;
                }
                err => {
                    err?;
                }
            }
        }

        Ok(None)
    }
}

mod environment;
mod globals;
pub mod error;
#[cfg(test)]
mod tests;

use crate::ast::{Expr, Stmt};
use crate::function::Function;
use crate::lox_value::LoxValue;
use crate::token_type::TokenType;

use self::environment::Environment;
use self::error::RuntimeError;

#[derive(Debug)]
pub struct Interpreter {
    environment: Box<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut environment = Environment::new();

        environment.values.insert(
            "clock".to_string(),
            LoxValue::Function(Function::new_native("clock".to_string(), 0, globals::clock)),
        );
        Interpreter {
            environment: Box::new(environment),
        }
    }

    /// Evaluates an expression.
    pub fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        use Expr::*;
        use LoxValue::*;
        use TokenType::*;

        match expr {
            Value {value, position:_ } => Ok(value.to_owned()),
            Grouping(inner_expr, position) => self.evaluate(inner_expr),
            &Expr::Identifier(name, position) => {
                let value = self.environment.get(&name);

                if value.is_some() {
                    Ok(value.unwrap().to_owned())
                } else {
                    Err(RuntimeError::VarDoesNotExist {
                        name,
                        position,
                    })
                }
            }
            &Assignment {
                name,
                value,
                position
            } => {
                let previous = self.environment.get(&name);

                if previous.is_none() {
                    Err(RuntimeError::VarDoesNotExist { name, position })
                } else {
                    let value = self.evaluate(value.as_ref())?;
                    self.environment.assign(name.as_str(), value);

                    Ok(LoxValue::Nil)
                }
            }
            &Unary { op, rhs, position } => match (&op.token_type, self.evaluate(rhs.as_ref())?) {
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
            Binary { lhs, op, rhs, position } => {
                let (lhs, rhs) = (self.evaluate(lhs)?, self.evaluate(rhs)?);

                match (&op.token_type, lhs, rhs) {
                    // Arithmetic Operators
                    (Plus, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Number(l + r))
                    }
                    (Minus, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Number(l - r))
                    }
                    (Star, LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Number(l * r))
                    }
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
                    _ => Err(RuntimeError::Generic(
                        "Don't really know".to_string(),
                        op.position,
                    )),
                }
            }
            Ternary {
                condition,
                result_1,
                result_2,
                postion,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.evaluate(result_1)
                } else {
                    self.evaluate(result_2)
                }
            }
            Call { callee, arguments, postion } => {
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

    /// Executes a statement.
    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        use Stmt::*;

        match statement {
            ExprStmt(expr) => {
                self.evaluate(expr)?;
            }
            PrintStmt(expr) => {
                println!("Raw: {}", expr);
                println!("{}", self.evaluate(expr)?);
            }
            Var { name, initializer } => {
                let initializer = self.evaluate(initializer)?;
                self.environment.define(name, initializer);
            }
            Block { declarations } => {
                // TODO This is some of the hackiest stuff I've ever written
                self.environment = Box::new(Environment::with_enclosing(self.environment.clone()));
                self.interpret(declarations)?;
                self.environment = self.environment.enclosing.clone().unwrap();
            }
            IfStmt {
                condition,
                true_stmt,
                false_stmt,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(&true_stmt)?
                } else if let Some(stmt) = false_stmt {
                    self.execute(&stmt)?
                }
            }
            WhileStmt { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
        }

        Ok(())
    }

    /// Executes statements given.
    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?
        }

        Ok(())
    }
}


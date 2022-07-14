use std::fmt;

use crate::lox_literal::LoxLiteral;

use crate::token_type::TokenType;

use crate::ast::{Expr, Stmt};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Evaluates an expression.
    pub fn evaluate(&self, expr: &Expr) -> Result<LoxLiteral, RuntimeError> {
        use Expr::*;

        match expr {
            Literal(value) => Ok(value.to_owned()),
            Grouping(inner_expr) => self.evaluate(inner_expr),
            Unary { op, rhs } => {
                use TokenType::*;

                match op.token_type {
                    Minus => match rhs.as_ref() {
                        Literal(LoxLiteral::Number(n)) => Ok(LoxLiteral::Number(-n)),
                        _ => Err(RuntimeError::Generic(
                            "Expected a number.".to_string(),
                            op.line,
                        )),
                    },
                    Bang => match rhs.as_ref() {
                        Literal(LoxLiteral::Boolean(b)) => Ok(LoxLiteral::Boolean(!b)),
                        _ => Err(RuntimeError::Generic(
                            "Expected a boolean expression".to_string(),
                            op.line,
                        )),
                    },
                    _ => Err(RuntimeError::Generic(
                        "Unexpected in unary section".to_string(),
                        op.line,
                    )),
                }
            }
            Binary { lhs, op, rhs } => {
                let (lhs, rhs) = (self.evaluate(lhs)?, self.evaluate(rhs)?);

                use TokenType::*;

                match (lhs, rhs, &op.token_type) {
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Plus) => {
                        Ok(LoxLiteral::Number(l + r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Minus) => {
                        Ok(LoxLiteral::Number(l - r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Star) => {
                        Ok(LoxLiteral::Number(l * r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Slash) => {
                        if r == 0.0 {
                            return Err(RuntimeError::DivisionByZero(op.line));
                        }
                        Ok(LoxLiteral::Number(l / r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), EqualEqual) => {
                        Ok(LoxLiteral::Boolean(l == r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), BangEqual) => {
                        Ok(LoxLiteral::Boolean(l != r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Greater) => {
                        Ok(LoxLiteral::Boolean(l > r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), GreaterEqual) => {
                        Ok(LoxLiteral::Boolean(l >= r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), Less) => {
                        Ok(LoxLiteral::Boolean(l < r))
                    }
                    (LoxLiteral::Number(l), LoxLiteral::Number(r), LessEqual) => {
                        Ok(LoxLiteral::Boolean(l <= r))
                    }

                    // String Concatenation
                    (LoxLiteral::String(s1), LoxLiteral::String(s2), Plus) => {
                        Ok(LoxLiteral::String(s1 + s2.as_str()))
                    }
                    _ => Err(RuntimeError::Generic(
                        "Don't really know".to_string(),
                        op.line,
                    )),
                }
            }
            Ternary {
                condition,
                result_1,
                result_2,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.evaluate(result_1)
                } else {
                    self.evaluate(result_2)
                }
            }
        }
    }

    fn execute(&mut self, statement: &Stmt) {
        use Stmt::*;
        match statement {
            PrintStmt(expr) => println!(
                "Output: {}",
                self.evaluate(&expr)
                    .unwrap_or_else(|err| panic!("Error: {}", err))
            ),
            ExprStmt(expr) => {
                self.evaluate(&expr)
                    .unwrap_or_else(|err| panic!("Error: {}", err));
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement)
        }
    }
}

pub enum RuntimeError {
    Generic(String, usize),
    DivisionByZero(usize),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            Generic(reason, line_no) => write!(f, "Error: {}, on line {}", reason, line_no),
            DivisionByZero(line_no) => {
                write!(f, "Error: Division by zero error, on line {}", line_no)
            }
        }
    }
}

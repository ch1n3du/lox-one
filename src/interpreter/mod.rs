mod enviroment;
mod runtime_error;

use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{Expr, Stmt};
use crate::lox_literal::LoxLiteral;
use crate::token_type::TokenType;

use self::enviroment::Enviroment;
use self::runtime_error::RuntimeError;

pub struct Interpreter {
    enviroment: Rc<RefCell<Enviroment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            enviroment: Rc::new(RefCell::new(Enviroment::new())),
        }
    }

    pub fn with_enclosing(&self) -> Interpreter {
        Interpreter { enviroment: Enviroment::with_enclosing(&self.enviroment) }
    }

    /// Evaluates an expression.
    pub fn evaluate(&self, expr: &Expr) -> Result<LoxLiteral, RuntimeError> {
        use Expr::*;
        use LoxLiteral::*;
        use TokenType::*;

        match expr {
            Literal(value) => Ok(value.to_owned()),
            Grouping(inner_expr) => self.evaluate(inner_expr),
            Expr::Identifier { name, line_no } => {
                let value = self.enviroment.borrow_mut().get(&name);

                if value.is_some() {
                    Ok(value.unwrap().to_owned())
                } else {
                    Err(RuntimeError::VarDoesNotExist {
                        name: name.to_owned(),
                        line_no: line_no.to_owned(),
                    })
                }
            }
            Unary { op, rhs } => match (&op.token_type, self.evaluate(rhs)?) {
                (Minus, LoxLiteral::Number(n)) => Ok(LoxLiteral::Number(-n)),
                (Minus, _) => Err(RuntimeError::Generic(
                    "Expected a number.".to_string(),
                    op.line,
                )),

                (Bang, Boolean(b)) => Ok(LoxLiteral::Boolean(!b)),
                (Bang, _) => Err(RuntimeError::Generic(
                    "Expected a boolean expression".to_string(),
                    op.line,
                )),

                _ => Err(RuntimeError::Generic(
                    "Expected a unary expression.".to_string(),
                    op.line,
                )),
            },
            Binary { lhs, op, rhs } => {
                let (lhs, rhs) = (self.evaluate(lhs)?, self.evaluate(rhs)?);

                match (&op.token_type, lhs, rhs) {
                    // Arithmetic Operators
                    (Plus, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Number(l + r))
                    }
                    (Minus, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Number(l - r))
                    }
                    (Star, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Number(l * r))
                    }
                    (Slash, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        if r == 0.0 {
                            return Err(RuntimeError::DivisionByZero(op.line));
                        }
                        Ok(LoxLiteral::Number(l / r))
                    }

                    // Logical Operators
                    (EqualEqual, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l == r))
                    }
                    (BangEqual, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l != r))
                    }
                    (Greater, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l > r))
                    }
                    (GreaterEqual, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l >= r))
                    }
                    (Less, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l < r))
                    }
                    (LessEqual, LoxLiteral::Number(l), LoxLiteral::Number(r)) => {
                        Ok(LoxLiteral::Boolean(l <= r))
                    }

                    // String Concatenation
                    (Plus, LoxLiteral::String(s1), LoxLiteral::String(s2)) => {
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

    /// Executes a statement.
    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        use Stmt::*;

        match statement {
            ExprStmt(expr) => {
                self.evaluate(expr)?;
            }
            PrintStmt(expr) => println!("{}", self.evaluate(expr)?),
            Var { name, initializer } => {
                let initializer = self.evaluate(initializer)?;
                self.enviroment.borrow_mut().define(name, initializer);
            },
            Block { declarations } => {
                let mut block_interpreter = self.with_enclosing();
                block_interpreter.interpret(declarations)?
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    use std::fs::File;
    use std::io::Read;

    fn assert_execution_of(src: &str, verbose: bool) -> Interpreter {
        let tokens = Scanner::tokens_from_str(src, verbose);

        let mut parser = Parser::new(tokens);
        let statements = parser.program()
            .unwrap_or_else(|err| panic!("Error in 'assert_execution_of':\nSourceCode:\n{}\nError:\n{}", src, err));

        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();

        interpreter
    }

    fn assert_execution_of_file(path: &str, verbose: bool) -> Interpreter {
        let mut file = File::open(path)
            .unwrap_or_else(|_| panic!("Error opening file in 'assert_execution_of_file'"));

        let mut src = String::new();
        file.read_to_string(&mut src)
            .unwrap_or_else(|_| panic!("Error reading file in 'assert_execution_of_file'"));

        assert_execution_of(src.as_str(), verbose)
    }

    #[test]
    fn executes_expr_statements() {
        assert_execution_of_file("examples/expr_stmt.lox", false);
    }

    #[test]
    fn executes_print_statements() {
        assert_execution_of_file("examples/print_stmt.lox", false);
   }

    #[test]
    fn executes_variables() {
        assert_execution_of_file("examples/variables.lox", false);
    }

    #[test]
    fn executes_block_statements() {
        assert_execution_of_file("examples/block_stmt.lox", false);
   }
}

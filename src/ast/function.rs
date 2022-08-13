use crate::interpreter::{error::InterpreterResult, Interpreter};
// use crate::
use super::{callable::Callable, lox_value::LoxValue, Expr, Stmt};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: String,
    pub params: Vec<Expr>,
    pub body: Box<Stmt>,
}

impl fmt::Display for FunDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone)]
pub enum Function {
    Native {
        name: &'static str,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxValue]) -> InterpreterResult<LoxValue>,
    },
    Local(FunDecl),
}

impl Function {
    pub fn new_native(
        name: &'static str,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxValue]) -> InterpreterResult<LoxValue>,
    ) -> Function {
        Function::Native {
            name,
            arity,
            callable,
        }
    }

    pub fn new_local(fun_declaration: &FunDecl) -> Function {
        Function::Local(fun_declaration.to_owned())
    }
}

impl Callable for Function {
    fn name(&self) -> String {
        use Function::*;

        match self {
            Native {
                name,
                arity,
                callable,
            } => name.to_string(),
            Local(FunDecl {
                name,
                params: _,
                body: _,
            }) => name.to_string(),
        }
    }

    fn arity(&self) -> usize {
        use Function::*;

        match self {
            Native {
                name,
                arity,
                callable,
            } => arity.to_owned(),
            Local(FunDecl {
                name: _,
                params,
                body: _,
            }) => params.len(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxValue],
    ) -> InterpreterResult<LoxValue> {
        use Function::*;

        match self {
            Native {
                name,
                arity,
                callable,
            } => callable(interpreter, args),
            Local(FunDecl { name, params, body }) => {
                todo!()
            }
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Function::*;

        match self {
            Native {
                name,
                arity: _,
                callable: _,
            } => {
                write!(f, "<native fn {}>", name)
            }
            Local(FunDecl { name, params, body }) => {
                todo!()
            }
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }

    fn ne(&self, other: &Self) -> bool {
        self.name() != other.name()
    }
}

use crate::{
    callable::Callable,
    interpreter::{error::RuntimeResult, Interpreter},
    lox_value::LoxValue, ast::Stmt,
};

use std::fmt;

use parse_display::Display;

#[derive(Debug, Display, Clone)]
#[display("Function Declaration:\n    Name: {name}\n    Params: {params:?}\n    Body: {body}")]
pub struct FunDecl {
    pub name: String,
    pub params: Vec<LoxValue>,
    pub body: Box<Stmt>,
}

#[derive(Display, Clone)]
pub enum Function {
    #[display("<native fun {name}>")]
    Native {
        name: String,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxValue]) -> RuntimeResult<LoxValue>,
    },
    #[display("User Function:\n{0}")]
    User(FunDecl)
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }

    fn ne(&self, other: &Self) -> bool {
        self.name() != other.name()
    }
}

impl Function {
    pub fn new_native_fun(
        name: String,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxValue]) -> RuntimeResult<LoxValue>,
    ) -> Function {
        Function::Native {
            name,
            arity,
            callable,
        }
    }
    pub fn new_user_fun(decl: FunDecl) -> Function {
        Function::User(decl)
    }
}

impl Callable for Function {
    fn name(&self) -> String {
        use Function::*;

        match self {
            Native {
                name,
                arity:_,
                callable:_,
            } => name.to_owned(),
            User(decl) => { 
                decl.name.to_owned()
            }
        }
    }

    fn arity(&self) -> usize {
        use Function::*;

        match self {
            Native {
                name:_,
                arity,
                callable:_,
            } => arity.to_owned(),
            User(decl) => {
                decl.params.len()
            }
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxValue],
    ) -> RuntimeResult<LoxValue> {
        use Function::*;

        match self {
            Native {
                name:_,
                arity:_,
                callable,
            } => callable(interpreter, args),
            User(decl) => {
                todo!()
            }
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Function::*;
        match self {
            Native { name, arity:_, callable:_ } => {
                write!(f, "<native fun {name}>")
            }
            User(decl) => {
                write!(f, "<user fun {}>", &decl.name)
            }
        }
    }
}
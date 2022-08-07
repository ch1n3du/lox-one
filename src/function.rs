use crate::{
    callable::Callable,
    interpreter::{runtime_error::RuntimeError, Interpreter},
    lox_literal::LoxLiteral,
};

use std::fmt;

#[derive(Clone)]
pub enum Function {
    Native {
        name: String,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxLiteral]) -> Result<LoxLiteral, RuntimeError>,
    },
}

impl Function {
    pub fn new_native(
        name: String,
        arity: usize,
        callable: fn(&mut Interpreter, &[LoxLiteral]) -> Result<LoxLiteral, RuntimeError>,
    ) -> Function {
        Function::Native {
            name,
            arity,
            callable,
        }
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
            } => name.to_owned(),
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
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxLiteral],
    ) -> Result<LoxLiteral, RuntimeError> {
        use Function::*;

        match self {
            Native {
                name,
                arity,
                callable,
            } => callable(interpreter, args),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Function::*;

        match self {
            Native {
                name,
                arity,
                callable,
            } => {
                write!(f, "<fn {}>", name)
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

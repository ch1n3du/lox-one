use crate::interpreter::error::InterpreterResult;
use crate::interpreter::Interpreter;

use super::lox_value::LoxValue;

pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue])
        -> InterpreterResult<LoxValue>;
}

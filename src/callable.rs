use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;

use crate::interpreter::error::RuntimeResult;

pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue]) -> RuntimeResult<LoxValue>;
}

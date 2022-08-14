use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;

use crate::interpreter::runtime_error::RuntimeError;

pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxValue],
    ) -> Result<LoxValue, RuntimeError>;
}

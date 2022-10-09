use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;

use crate::interpreter::error::RuntimeResult;

/// This trait is shared among all types that can be called
/// like functions and class instantiations.
pub trait Callable {
    /// Returns the name of the callable.
    fn name(&self) -> String;
    /// Returns the number of arguments taken by the callable.
    fn arity(&self) -> usize;
    /// Takes a `&mut Interpreter` and calls the function on it.
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxValue]) -> RuntimeResult<LoxValue>;
}

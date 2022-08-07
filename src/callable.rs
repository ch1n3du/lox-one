use crate::interpreter::Interpreter;
use crate::lox_literal::LoxLiteral;

use crate::interpreter::runtime_error::RuntimeError;

pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxLiteral],
    ) -> Result<LoxLiteral, RuntimeError>;
}

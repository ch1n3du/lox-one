use crate::lox_value::LoxValue;

use crate::interpreter::{error::RuntimeError, Interpreter};

pub fn clock(
    _interpreter: &mut Interpreter,
    _args: &[LoxValue],
) -> Result<LoxValue, RuntimeError> {
    Ok(LoxValue::Number(13124312.0))
}

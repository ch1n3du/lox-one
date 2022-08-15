use crate::lox_value::LoxValue;

use crate::interpreter::{error::RuntimeError, Interpreter};

pub fn clock(
    interpreter: &mut Interpreter,
    args: &[LoxValue],
) -> Result<LoxValue, RuntimeError> {
    Ok(LoxValue::Number(13124312.0))
}

use crate::lox_literal::LoxLiteral;

use super::{runtime_error::RuntimeError, Interpreter};

pub fn clock(
    interpreter: &mut Interpreter,
    args: &[LoxLiteral],
) -> Result<LoxLiteral, RuntimeError> {
    Ok(LoxLiteral::Number(13124312.0))
}

use crate::lox_value::LoxValue;

use crate::interpreter::{error::RuntimeResult, Interpreter};


pub fn clock(_interpreter: &mut Interpreter, _args: &[LoxValue]) -> RuntimeResult<LoxValue> {
    Ok(LoxValue::Number(13124312.0))
}

pub fn _print(_interpreter: &mut Interpreter, args: &[LoxValue]) -> RuntimeResult<LoxValue> {
    println!("{}", args[0]);
    Ok(LoxValue::Nil)
}
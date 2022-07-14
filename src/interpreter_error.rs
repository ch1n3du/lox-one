use std::fmt;

pub enum InterpreterError {
    GenericError { line_no: usize, reason: String },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InterpreterError::*;
        match self {
            GenericError { line_no, reason } => write!(f, "Error: {}, on line {}", reason, line_no),
        }
    }
}

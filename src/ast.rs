use std::fmt::{self, Debug};

use crate::lox_literal::LoxLiteral;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LoxLiteral),
    Grouping(Box<Expr>),
    Unary {
        op: Token,
        rhs: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        result_1: Box<Expr>,
        result_2: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;

        match self {
            Literal(literal) => write!(f, "{}", literal),
            Grouping(grouping) => write!(f, "(grouping {})", grouping),
            Unary { op, rhs } => write!(f, "({} {})", op.token_type, rhs),
            Binary { lhs, op, rhs } => write!(f, "({} {} {})", op.token_type, lhs, rhs),
            Ternary {
                condition,
                result_1,
                result_2,
            } => write!(f, "(ternary {} ? {} : {})", condition, result_1, result_2),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    PrintStmt(Expr),
    ExprStmt(Expr),
}

#[cfg(test)]
mod tests {}

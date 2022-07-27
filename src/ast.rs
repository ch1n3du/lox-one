use std::fmt::{self, format, Debug};

use crate::lox_literal::LoxLiteral;
use crate::token::Token;
use crate::token_type::TokenType;

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
    Identifier {
        name: String,
        line_no: usize,
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
            Identifier { name, line_no } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    PrintStmt(Expr),
    ExprStmt(Expr),
    Var { name: String, initializer: Expr },
    Block { declarations: Vec<Stmt> },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {
            PrintStmt(expr) => write!(f, "print {};", expr),
            ExprStmt(expr) => write!(f, "{};", expr),
            Var { name, initializer } => write!(f, "var {} = {}", name, initializer),
            Block { declarations } => {
                let repr = declarations
                    .iter()
                    .fold(String::from("\n{\n"), |acc, stmt| {
                        format!("{}\t{}\n", acc, stmt)
                    });

                write!(f, "{}}}", repr)
            }
        }
    }
}

#[cfg(test)]
mod tests {}

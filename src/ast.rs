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
    Identifier {
        name: String,
        line_no: usize,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
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
            Identifier { name, line_no: _ } => write!(f, "{}", name),
            Assignment {
                name,
                value,
                line_no: _,
            } => write!(f, "{} = {}", name, value),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    PrintStmt(Expr),
    ExprStmt(Expr),
    Var {
        name: String,
        initializer: Expr,
    },
    Block {
        declarations: Vec<Stmt>,
    },
    IfStmt {
        condition: Expr,
        true_stmt: Box<Stmt>,
        false_stmt: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {
            PrintStmt(expr) => write!(f, "print {};", expr),
            ExprStmt(expr) => write!(f, "{};", expr),
            Var { name, initializer } => write!(f, "var {} = {}", name, initializer),
            Block { declarations } => {
                let repr = declarations.iter().fold(String::from("{\n"), |acc, stmt| {
                    format!("{}    {}\n", acc, stmt)
                });

                write!(f, "{}}}", repr)
            }
            IfStmt {
                condition,
                true_stmt,
                false_stmt,
            } => match false_stmt {
                None => write!(f, "if ({}) {}", condition, true_stmt,),
                Some(stmt) => write!(f, "if ({}) {} else {}", condition, true_stmt, stmt),
            },
            WhileStmt { condition, body } => write!(f, "while ({}) {}", condition, body),
        }
    }
}

#[cfg(test)]
mod tests {}

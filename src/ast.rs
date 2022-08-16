use crate::function::FunDecl;
use crate::lox_value::LoxValue;

use crate::token::{Position, Token};

use std::fmt::{self, Debug};

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String, Position),
    Value {
        value: LoxValue,
        position: Position,
    },
    Grouping(Box<Expr>, Position),
    Unary {
        op: Token,
        rhs: Box<Expr>,
        position: Position,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
        position: Position,
    },
    Ternary {
        condition: Box<Expr>,
        result_1: Box<Expr>,
        result_2: Box<Expr>,
        postion: Position,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
        position: Position,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        postion: Position,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;

        match self {
            Value { value, position:_ } => write!(f, "{}", value),
            Grouping (expr, _position ) => write!(f, "({})", expr),
            Unary { op, rhs, position:_ } => write!(f, "{} {}", op.token_type, rhs),
            Binary {
                lhs,
                op,
                rhs,
                position:_,
            } => write!(f, "{} {} {}", lhs, op.token_type, rhs),
            Ternary {
                condition,
                result_1,
                result_2,
                postion:_,
            } => write!(f, "{} ? {} : {}", condition, result_1, result_2),
            Identifier (name, _position ) => write!(f, "{}", name),
            Assignment {
                name,
                value,
                position:_,
            } => write!(f, "{} = {}", name, value),
            Call {
                callee,
                arguments,
                postion:_,
            } => {
                let args = if arguments.len() == 0 {
                    String::new()
                } else {
                    arguments
                        .iter()
                        .fold(format!("{}", arguments[0]), |acc, arg| {
                            format!("{}, {}", acc, arg)
                        })
                };

                write!(f, "{}({})", callee, args)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    PrintStmt(Expr),
    ExprStmt(Expr),
    Var {
        name: String,
        initializer: Expr,
        postion: Position,
    },
    Block(Vec<Stmt>),
    IfStmt {
        condition: Expr,
        true_stmt: Box<Stmt>,
        false_stmt: Option<Box<Stmt>>,
        position: Position,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
        position: Position,
    },
    BreakStmt(Position),
    ContinueStmt(Position),
    FunStmt {
        fun_declaration: FunDecl,
        position: Position,
    },
    ReturnStmt {
        expr: Option<Expr>,
        position: Position,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;

        match self {
            PrintStmt ( expr ) => write!(f, "print {};", expr),
            ExprStmt(expr) => write!(f, "{};", expr),
            Var {
                name,
                initializer,
                postion:_,
            } => write!(f, "var {} = {}", name, initializer),
            Block(declarations) => {
                let repr = declarations.iter().fold(String::from("{\n"), |acc, stmt| {
                    format!("{}    {}\n", acc, stmt)
                });

                write!(f, "{}}}", repr)
            }
            IfStmt {
                condition,
                true_stmt,
                false_stmt,
                position:_,
            } => match false_stmt {
                None => write!(f, "if ({}) {}", condition, true_stmt,),
                Some(stmt) => write!(f, "if ({}) {} else {}", condition, true_stmt, stmt),
            },
            WhileStmt {
                condition,
                body,
                position:_,
            } => write!(f, "while ({}) {}", condition, body),
            BreakStmt(_position) => write!(f, "break ;"),
            ContinueStmt(_position) => write!(f, "continue ;"),
            FunStmt {
                fun_declaration: FunDecl { name, params, body },
                position:_,
            } => {
                let params_repr = if params.len() == 0 {
                    String::new()
                } else {
                    params.iter().fold(format!("{}", params[0]), |acc, arg| {
                        format!("{}, {}", acc, arg)
                    })
                };

                write!(f, "fun {}({}) {}", name, params_repr, body)
            }
            ReturnStmt { expr, position:_ } => {
                if let Some(expr) = expr {
                    write!(f, "return {};", expr)
                } else {
                    write!(f, "return ;")
                }
            }
        }
    }
}

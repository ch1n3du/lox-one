use std::fmt;

use crate::lox_literal::LoxLiteral;
use crate::token::Token;

pub enum Operator {
}

pub enum Expression {
    Literal(LoxLiteral),
    Operator(Token),
    Unary {
        postfix_operator: Token,
        right: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        operator: Token,
        rhs: Box<Expression>,
    }, 
    Grouping(Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        match self {
            Literal(l) => write!(f, "{}", l),
            Operator(Token) => {
                todo!()
            },
            todo!()
        }
    }
}

pub enum Ast {
    Expression(Expression)
}
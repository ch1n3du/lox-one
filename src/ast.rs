use std::fmt;

use crate::lox_literal::LoxLiteral;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LoxLiteral),
    Operator(TokenType),
    Grouping(Box<Expression>),
    Unary {
        prefix_op: Token,
        rhs: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        infix_op: Token,
        rhs: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        match self {
            Literal(literal) => write!(f, "{}", literal),
            Operator(token_type) => write!(f, "{}", token_type),
            Grouping(grouping) => write!(f, "(grouping {})", grouping),
            Unary { prefix_op, rhs } => write!(f, "({} {})", prefix_op.token_type, rhs),
            Binary { lhs, infix_op, rhs } => write!(f, "({} {} {})", infix_op.token_type, lhs, rhs),
        }
    }
}

#[derive(Debug)]
pub enum Ast {
    Expression(Expression),
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Ast::*;

        match self {
            Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Ast, Expression},
        lox_literal::LoxLiteral,
        token_type::TokenType,
    };
}

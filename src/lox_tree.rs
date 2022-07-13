use std::fmt;

use crate::lox_literal::LoxLiteral;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum LoxTree {
    Literal(LoxLiteral),
    Operator(TokenType),
    Grouping(Box<LoxTree>),
    Unary {
        prefix_op: Token,
        rhs: Box<LoxTree>,
    },
    Binary {
        lhs: Box<LoxTree>,
        infix_op: Token,
        rhs: Box<LoxTree>,
    },
    Ternary {
        condition: Box<LoxTree>,
        result_1: Box<LoxTree>,
        result_2: Box<LoxTree>,
    },
}

impl fmt::Display for LoxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxTree::*;

        match self {
            Literal(literal) => write!(f, "{}", literal),
            Operator(token_type) => write!(f, "{}", token_type),
            Grouping(grouping) => write!(f, "(grouping {})", grouping),
            Unary { prefix_op, rhs } => write!(f, "({} {})", prefix_op.token_type, rhs),
            Binary { lhs, infix_op, rhs } => write!(f, "({} {} {})", infix_op.token_type, lhs, rhs),
            Ternary {
                condition,
                result_1,
                result_2,
            } => write!(f, "(ternary {} ? {} : {})", condition, result_1, result_2),
        }
    }
}

#[cfg(test)]
mod tests {}

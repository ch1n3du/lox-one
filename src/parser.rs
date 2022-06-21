/*
"Grammar, which knows how to control even kings."
                                    - Moliere
*/

use crate::{
    ast::{Ast, Expression},
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            start: 0,
            current: 0,
        }
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        Some(self.tokens[self.current].clone())
    }

    fn previous(&self) -> Option<Token> {
        if self.current == 0 {
            return None;
        }
        Some(self.tokens[self.current - 1].clone())
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        self.increment_current();
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        match self.peek() {
            Some(token) => &token.token_type == token_type,
            None => false,
        }
    }

    /// Checks if next Token matches any of the Tokens in tokens
    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in &token_types {
            if self.check(token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil"
    ///          | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expression, ParserError> {
        let curr_token = self.peek().unwrap();

        if self.matches(vec![
            TokenType::Number,
            TokenType::String,
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
        ]) {
            return Ok(Expression::Literal(
                curr_token.literal.unwrap(),
            ));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            
        }

        Err(ParserError::Iono)
    }

    /// unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let prefix_op = self.previous().unwrap();
            let rhs = self.expression()?;

            return Ok(Expression::Unary { prefix_op, rhs: Box::new(rhs)});
        }

        Err(ParserError::Iono)
    }

    /// factor -> unary ( ( "/" | "*" ) factor )* ;
    fn factor(&mut self) -> Result<Expression, ParserError> {
        let expr = self.unary()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let infix_op = self.previous();

            let rhs = self.factor()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                infix_op: infix_op.unwrap(),
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// TODO ODODODODODODODODODODODODODODODODO
    /// term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    /// expression -> equality
    fn expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }
}

pub enum ParserError {
    Iono,
    Hmmm,
    Eof,
}

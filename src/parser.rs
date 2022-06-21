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

    /// Checks if self.current is at the end.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Returns the next token without consuming it.
    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        Some(self.tokens[self.current].clone())
    }

    /// Return the token before the current token.
    fn previous(&self) -> Option<Token> {
        if self.current == 0 {
            return None;
        }
        Some(self.tokens[self.current - 1].clone())
    }

    /// Gets the line number of the previous token.
    fn get_line_no(&self) -> usize {
        self.previous().unwrap().line
    }

    /// Return the token at current and increments self.current.
    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        self.increment_current();
        self.previous()
    }

    /// Checks if token_type matches the token type of the next token without modifying self.current.
    fn check(&self, token_type: &TokenType) -> bool {
        match self.peek() {
            Some(token) => &token.token_type == token_type,
            None => false,
        }
    }

    /// Checks if any of the token_types matches the token_type of the next token
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
            return Ok(Expression::Literal(curr_token.literal.unwrap()));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;

            if self.matches(vec![TokenType::RightParen]) {
                return Ok(Expression::Grouping(Box::new(expr)))
            } else {
                return Err(ParserError::ExpectedClosingBrace(self.get_line_no()))
            };
        }

        Err(ParserError::UnexpectedToken(self.get_line_no()))
    }

    /// unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let prefix_op = self.previous().unwrap();
            let rhs = self.unary()?;

            // println!("In unary: \nPREFIX {} \nRHS: {}", prefix_op, rhs)
            return Ok(Expression::Unary {
                prefix_op,
                rhs: Box::new(rhs),
            });
        } else {
            self.primary()
        }
    }

    /// factor -> unary ( ( "/" | "*" ) factor )* ;
    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let infix_op = self.previous().unwrap();

            let rhs = self.factor()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                infix_op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// TODO ODODODODODODODODODODODODODODODODO
    /// term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let infix_op = self.previous().unwrap();

            let rhs = self.factor()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                infix_op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.term()?;

        while self.matches(vec![TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual]) {
            let infix_op = self.previous().unwrap();

            let rhs = self.term()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                infix_op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison)
    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let infix_op = self.previous().unwrap();

            let rhs = self.comparison()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                infix_op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// expression -> equality
    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.equality()
    }

}

#[derive(Debug, Clone)]
pub enum ParserError {
    ExpectedOneOf(usize, Vec<String>),
    ExpectedClosingBrace(usize),
    UnexpectedToken(usize),
    // Iono(usize, String),
    // Hmmm,
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{Scanner, self};

    #[test]
    fn parses() {
        let input = "5 * (4 * 3) / 3 / 7";
        println!("\nInput String: \n================ \n'{}'\n", input);

        let tokens = Scanner::tokens_from_str(input, true);

        let expr = Parser::new(tokens).expression();
        println!("RAW: \n\n{:?}\n\n", expr.clone().unwrap());
        println!("PRETTY_PRINTING: \n\n{}\n", expr.clone().unwrap());
    }
}
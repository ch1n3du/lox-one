/*
"Grammar, which knows how to control even kings."
                                    - Moliere
*/

use crate::token::Token;
use crate::token_type::TokenType;

use crate::ast::Expr;
use crate::lox_literal::LoxLiteral;

use crate::parser_errors::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }

    /// Checks if self.current is at the end.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
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
        if self.check(&TokenType::Semicolon) {
            self.increment_current();
            let tok = self.previous();
            self.increment_current();
            return tok;
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

    /// Checks if token_type matches the token type of the next token without modifying self.current.
    fn check_prev(&self, token_type: &TokenType) -> bool {
        match self.previous() {
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

    /// Consumes a token if it matches token_type else returns.
    fn consume(
        &mut self,
        token_type: &TokenType,
        error: ParserError,
    ) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return match self.advance() {
                Some(token) => Ok(token),
                None => Err(ParserError::Eof(self.get_line_no())),
            };
        }

        println!("Got here.");

        Err(error)
    }

    /// Synchronizes on error.
    /// While current is not at end
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.check_prev(&TokenType::Semicolon) {
                return;
            }

            match self.peek() {
                Some(token) => match token.token_type {
                    TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return => return,

                    _ => (),
                },
                None => return,
            }

            self.advance();
        }
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil"
    ///          | "("Expr  ")" ;
    fn primary(&mut self) -> Result<Expr, ParserError> {
        // let curr_token = self.peek().unwrap();

        if self.matches(vec![TokenType::Semicolon]) {
            return Ok(Expr::Literal(LoxLiteral::Nil));
        }

        if self.matches(vec![
            TokenType::Number,
            TokenType::String,
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
        ]) {
            return Ok(Expr::Literal(self.previous().unwrap().literal.unwrap()));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                &TokenType::RightParen,
                ParserError::ExpectedClosingBrace(self.get_line_no()),
            )?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        match self.consume(&TokenType::Eof, ParserError::Eof(self.get_line_no())) {
            Ok(_) => Err(ParserError::Eof(self.get_line_no())),
            Err(e) => Err(e),
        }

        // Err(ParserError::UnexpectedToken(self.get_line_no(), self.previous().unwrap()))
    }

    /// unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().unwrap();
            let rhs = self.unary()?;

            // println!("In unary: \nPREFIX {} \nRHS: {}", prefix_op, rhs)
            Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs),
            })
        } else {
            self.primary()
        }
    }

    /// factor -> unary ( ( "/" | "*" ) factor )* ;
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous().unwrap();

            let rhs = self.factor()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// TODO ODODODODODODODODODODODODODODODODO
    /// term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().unwrap();

            let rhs = self.factor()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.matches(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let op = self.previous().unwrap();

            let rhs = self.term()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison)*
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().unwrap();

            let rhs = self.comparison()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// ternary ->Expr  ( "?" ternary ":" ternary )?
    fn ternary(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        if self.matches(vec![TokenType::QuestionMark]) {
            let result_1 = self.ternary()?;

            if self.matches(vec![TokenType::Colon]) {
                let result_2 = self.ternary()?;
                expr = Expr::Ternary {
                    condition: Box::new(expr),
                    result_1: Box::new(result_1),
                    result_2: Box::new(result_2),
                };

                return Ok(expr);
            } else {
                return Err(ParserError::ExpectedOneOf(
                    self.get_line_no(),
                    vec![TokenType::Colon],
                ));
            }
        }

        Ok(expr)
    }

    ///Expr  -> equality
    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        let expr = self.ternary()?;
        // println!("Before consume");
        // self.consume(&TokenType::Semicolon, ParserError::ExpectedOneOf(self.get_line_no(), vec![TokenType::Semicolon]));
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{parser_errors::ParserError, scanner::Scanner, token_type::TokenType};

    #[test]
    fn parses() {
        // let input = "5 * (4 * 3) / 3 / 7";
        let input = "true ? 43  34";

        println!("\nInput String: \n================ \n'{}'\n", input);

        let tokens = Scanner::tokens_from_str(input, false);

        let expr = Parser::new(tokens).expression();

        match expr {
            Ok(expr) => println!("Success: {}", expr),
            Err(err) => println!("Error: {}", err),
        }

        println!(
            "{}",
            ParserError::ExpectedOneOf(1, vec![TokenType::Colon, TokenType::Dot,])
        );
        // println!("\nRAW: \n{:?}\n", expr.clone());
        // println!("PRETTY_PRINTING: \n{}\n", expr.clone().unwrap());
    }
}

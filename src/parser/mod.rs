/*
"Grammar, which knows how to control even kings."
                                    - Moliere
*/

mod parser_error;

use crate::token::Token;
use crate::token_type::TokenType;

use crate::ast::{Expr, Stmt};
use crate::lox_literal::LoxLiteral;

use parser_error::ParserError;

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
        self.current > self.tokens.len() - 2
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
    fn line_no(&self) -> usize {
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
        match (self.check(token_type), self.advance()) {
            (true, Some(token)) => Ok(token),
            (true, None) => Err(ParserError::Eof(self.line_no())),
            _ => Err(error),
        }
    }

    /// Consumes a semicolon
    fn consume_semicolon(&mut self) -> Result<(), ParserError> {
        self.consume(
            &TokenType::Semicolon,
            ParserError::ExpectedOneOf(self.line_no(), vec![TokenType::Semicolon]),
        )?;

        Ok(())
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
    ///          | "("Expr  ")" | IDENTIFIER                  ;
    fn primary(&mut self) -> Result<Expr, ParserError> {
        // let curr_token = self.peek().unwrap();

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
                ParserError::ExpectedClosingBrace(self.line_no()),
            )?;

            return Ok(Expr::Grouping(Box::new(expr)));
        } else if self.matches(vec![TokenType::Identifier]) {
            let tok = self.previous().unwrap();
            let literal = tok.literal.unwrap();

            match literal {
                LoxLiteral::Identifier(name) => Ok(Expr::Identifier {
                    name,
                    line_no: tok.line,
                }),
                _ => panic!("This should be impossible {}", literal),
            }
        } else {
            println!("Invalid Token in primary rule: {:?}", self.peek());
            Err(ParserError::ExpectedOneOf(
                self.line_no(),
                vec![
                    TokenType::Number,
                    TokenType::String,
                    TokenType::False,
                    TokenType::True,
                    TokenType::False,
                ],
            ))
        }

        // panic!("\nOh no! \n\tCurrent: {:?}\n\tNext: {:?}", self.previous(), self.peek())
    }

    /// unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().unwrap();
            let rhs = self.unary()?;

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

    ///! Breaks the parser "32 ;\n false ? 32 : 323;\n";
    /// ternary -> expression ( "?" ternary ":" ternary )?
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
                    self.line_no(),
                    vec![TokenType::Colon],
                ));
            }
        }

        Ok(expr)
    }

    /// expression -> ternary
    fn expression(&mut self) -> Result<Expr, ParserError> {
        let expr = self.ternary()?;
        Ok(expr)
    }

    /// exprStatement  -> expression ";";
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        Ok(Stmt::ExprStmt(self.expression()?))
    }

    /// printStatement -> "print" expression ";";
    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        Ok(Stmt::PrintStmt(self.expression()?))
    }

    /// block -> "{" declaration* "}";
    fn block(&mut self) -> Result<Stmt, ParserError> {
        let mut declarations: Vec<Stmt> = Vec::new();

        while !self.matches(vec![TokenType::RightBrace]) {
            declarations.push(self.declaration()?)
        }

        Ok(Stmt::Block { declarations })
    }

    /// statement -> exprStmt
    ///           |  printStmt
    ///           |  block   ;
    pub fn statement(&mut self) -> Result<Stmt, ParserError> {
        let stmt = if self.matches(vec![TokenType::Print]) {
            self.print_statement()?
        } else if self.matches(vec![TokenType::LeftBrace]) {
            self.block()?
        } else {
            self.expression_statement()?
        };

        self.consume_semicolon()?;

        Ok(stmt)
    }

    /// varDeclaration -> "var" IDENTIFIER ("=" expression)?;
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let literal = self.advance().unwrap().literal.unwrap();

        let name = match literal {
            LoxLiteral::Identifier(s) => s,
            _ => {
                return Err(ParserError::ExpectedOneOf(
                    self.line_no(),
                    vec![TokenType::Identifier],
                ))
            }
        };

        let initializer = if self.matches(vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(LoxLiteral::Nil)
        };

        self.consume_semicolon()?;

        Ok(Stmt::Var { name, initializer })
    }

    /// declaration -> varDeclaration
    ///              | statement      ;
    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.matches(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    /// program -> declaration* EOF
    pub fn program(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod tests {
    // use super::Parser;
    // use crate::{parser_errors::ParserError, scanner::Scanner, token_type::TokenType};
}
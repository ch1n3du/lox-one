/*
"Grammar, which knows how to control even kings."
           &                         - Moliere
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
    fn consume(&mut self, token_type: TokenType) -> Result<Token, ParserError> {
        match (self.check(&token_type), self.advance()) {
            (true, Some(token)) => Ok(token),
            (true, None) => Err(ParserError::Eof(self.line_no())),
            _ => {
                println!("Wrong Token: {:?}", self.previous().unwrap());
                Err(ParserError::ExpectedOneOf {
                    line_no: self.line_no(),
                    token_types: vec![token_type],
                })
            }
        }
    }

    /// Synchronizes on error.
    /// While current is not at end
    fn synchronize(&mut self) {
        println!("Called synchronize");
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
            self.consume(TokenType::RightParen)?;

            return Ok(Expr::Grouping(Box::new(expr)));
        } else if self.matches(vec![TokenType::Identifier]) {
            let tok = self.previous().unwrap();
            let literal = tok.literal.unwrap();

            match literal {
                LoxLiteral::Identifier(name) => Ok(Expr::Identifier {
                    name,
                    line_no: tok.line,
                }),
                _ => {
                    return Err(ParserError::ExpectedOneOf {
                        line_no: self.line_no(),
                        token_types: vec![TokenType::Identifier],
                    })
                }
            }
        } else {
            println!(
                "Invalid Token in primary rule: '{}'",
                self.peek().unwrap().token_type
            );
            Err(ParserError::ExpectedOneOf {
                line_no: self.line_no(),
                token_types: vec![
                    TokenType::Number,
                    TokenType::String,
                    TokenType::False,
                    TokenType::True,
                    TokenType::False,
                ],
            })
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
                return Err(ParserError::ExpectedOneOf {
                    line_no: self.line_no(),
                    token_types: vec![TokenType::Colon],
                });
            }
        }

        Ok(expr)
    }

    /// logical_and -> ternary ("and" ternary)* ;
    fn logical_and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.ternary()?;

        while self.matches(vec![TokenType::And]) {
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: self.previous().unwrap(),
                rhs: Box::new(self.ternary()?),
            };
        }

        Ok(expr)
    }

    /// logical_or -> logic_and ("and" logic_and)* ;
    fn logical_or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.logical_and()?;

        while self.matches(vec![TokenType::Or]) {
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: self.previous().unwrap(),
                rhs: Box::new(self.logical_and()?),
            };
        }

        Ok(expr)
    }

    /// assignment -> IDENTIFIER "=" assignment
    ///             | logical_or ;
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.logical_or()?;

        match &expr {
            Expr::Identifier { name, line_no :_ } => {
                if self.matches(vec![TokenType::Equal]) {
                    let value = Box::new(self.assignment()?);

                    Ok(Expr::Assignment {
                        name: name.clone(),
                        value,
                        line_no: self.line_no(),
                    })
                } else {
                    Ok(expr)
                }
            }
            _ => Ok(expr),
        }
    }

    /// expression -> logical_or
    ///            |  assignment ;
    fn expression(&mut self) -> Result<Expr, ParserError> {
        let expr = self.assignment()?;
        Ok(expr)
    }

    /// exprStatement  -> expression ";";
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let stmt = Stmt::ExprStmt(self.expression()?);
        self.consume(TokenType::Semicolon)?;

        Ok(stmt)
    }

    /// printStatement -> "print" expression ";";
    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let stmt = Stmt::PrintStmt(self.expression()?);
        self.consume(TokenType::Semicolon)?;

        Ok(stmt)
    }

    /// block -> "{" declaration* "}" ;
    fn block(&mut self) -> Result<Stmt, ParserError> {
        let mut declarations: Vec<Stmt> = Vec::new();

        while !self.matches(vec![TokenType::RightBrace]) {
            declarations.push(self.declaration()?)
        }

        Ok(Stmt::Block { declarations })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen)?;

        let true_stmt = Box::new(self.statement()?);

        let false_stmt = if self.matches(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            condition,
            true_stmt,
            false_stmt,
        })
    }

    /// whileStmt -> "while" "(" expression ")" statement;
    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen)?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::WhileStmt { condition, body })
    }

    /// statement -> exprStmt
    ///           |  printStmt
    ///           |  block   
    ///           |  ifStmt
    ///           |  whileStmt ;
    pub fn statement(&mut self) -> Result<Stmt, ParserError> {
        let stmt = if self.matches(vec![TokenType::Print]) {
            self.print_statement()?
        } else if self.matches(vec![TokenType::LeftBrace]) {
            self.block()?
        } else if self.matches(vec![TokenType::If]) {
            self.if_statement()?
        } else if self.matches(vec![TokenType::While]) {
            self.while_statement()?
        } else {
            self.expression_statement()?
        };

        Ok(stmt)
    }

    /// varDeclaration -> "var" IDENTIFIER ("=" expression)?;
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let literal = self.advance().unwrap().literal.unwrap();

        let name = match literal {
            LoxLiteral::Identifier(s) => s,
            _ => {
                return Err(ParserError::ExpectedOneOf {
                    line_no: self.line_no(),
                    token_types: vec![TokenType::Identifier],
                })
            }
        };

        let initializer = if self.matches(vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(LoxLiteral::Nil)
        };

        self.consume(TokenType::Semicolon)?;

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
    pub fn program(&mut self) -> (Vec<Stmt>, Vec<ParserError>) {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<ParserError> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    // On an error synchronise the parser and continue
                    self.synchronize();
                    errors.push(err);
                }
            }
        }

        (statements, errors)
    }
}

#[cfg(test)]
mod tests {
    // use super::Parser;
    // use crate::{parser_errors::ParserError, scanner::Scanner, token_type::TokenType};
    use super::*;

    use crate::parser::Parser;
    use crate::scanner::Scanner;

    use crate::utils::{log_items, read_file};

    fn assert_can_parse(title: &str, src: &str, verbose: bool) -> (Vec<Stmt>, Vec<ParserError>) {
        let tokens = Scanner::tokens_from_str(src, verbose);

        let mut parser = Parser::new(tokens);
        let (statements, errors) = parser.program();

        if errors.len() != 0 {
            log_items(title, &errors)
        }

        (statements, errors)
    }

    fn assert_can_parse_file(path: &str, verbose: bool) -> (Vec<Stmt>, Vec<ParserError>) {
        let src = read_file(path);

        assert_can_parse(
            format!("Errors parsing '{}' file", path).as_str(),
            src.as_str(),
            verbose,
        )
    }

    #[test]
    fn can_parse_expr_statements() {
        assert_can_parse_file("examples/expr_stmt.lox", false);
    }

    #[test]
    fn can_parse_print_statements() {
        assert_can_parse_file("examples/print_stmt.lox", false);
    }

    #[test]
    fn can_parse_variable_declarations() {
        assert_can_parse_file("examples/variables.lox", false);
    }

    #[test]
    fn can_parse_assignment_expressions() {
        assert_can_parse_file("examples/assignment.lox", false);
    }

    #[test]
    fn can_parse_block_statements() {
        assert_can_parse_file("examples/variables.lox", false);
    }

    #[test]
    fn can_parse_if_statements() {
        assert_can_parse_file("examples/if_stmt.lox", false);
    }

    #[test]
    fn can_parse_if_else_statements() {
        assert_can_parse_file("examples/if_else_stmt.lox", false);
    }

    #[test]
    fn can_parse_logical_and() {
        assert_can_parse_file("examples/logic_and.lox", false);
    }

    #[test]
    fn can_parse_logical_or() {
        assert_can_parse_file("examples/logic_or.lox", false);
    }

    #[test]
    fn can_parse_while_statements() {
        assert_can_parse_file("examples/while_stmt.lox", false);
    }
}

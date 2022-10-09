/*
"Grammar, which knows how to control even kings."
                                     - Moliere
*/

pub mod error;
#[cfg(test)]
mod tests;

use crate::error::{LoxError, LoxResult};
use crate::function::FunDecl;
use crate::scanner::Scanner;
use crate::token::{Position, Token};
use crate::token_type::TokenType;

use crate::ast::{Expr, Stmt};
use crate::lox_value::LoxValue;

use error::ParserError;

use self::error::ParserResult;

#[derive(Debug)]
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
    fn position(&self) -> Position {
        self.previous().unwrap().position
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
    fn consume(&mut self, token_type: TokenType, msg: &str) -> ParserResult<Token> {
        match (self.check(&token_type), self.advance()) {
            (true, Some(token)) => Ok(token),
            (true, None) => Err(ParserError::Eof(self.position())),
            _ => {
                let tok = self.previous().unwrap();
                Err(ParserError::Expected {
                    msg: msg.to_string(),
                    found: tok.token_type,
                    position: self.position(),
                })
            }
        }
    }

    fn prev_token_type(&mut self) -> TokenType {
        self.previous().unwrap().token_type
    }

    /// Synchronizes on error.
    /// While current is not at end
    fn synchronize(&mut self) {
        // ! UNCOMMENT FOR DEBUGGING
        // println!("Called synchronize");
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
    fn primary(&mut self) -> ParserResult<Expr> {
        // let curr_token = self.peek().unwrap();

        if self.matches(vec![
            TokenType::Number,
            TokenType::String,
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
        ]) {
            return Ok(Expr::Value {
                value: self.previous().unwrap().literal.unwrap(),
                position: self.position(),
            });
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expected accompanying  closing bracket ')'",
            )?;

            return Ok(Expr::Grouping(Box::new(expr), self.position()));
        } else if self.matches(vec![TokenType::Identifier]) {
            let tok = self.previous().unwrap();
            let literal = tok.literal.unwrap();

            match literal {
                LoxValue::Identifier(name) => Ok(Expr::Identifier(name, self.position())),
                _ => Err(ParserError::Expected {
                    msg: "This should be impossible, check 'primary' parse rule.".to_string(),
                    found: tok.token_type,
                    position: self.position(),
                }),
            }
        } else {
            Err(ParserError::Expected {
                msg: "Expected a number, string or boolean value ".to_string(),
                found: self.peek().unwrap().token_type,
                position: self.position(),
            })
        }
    }

    /// arguments -> expression ( "," expression )* ;
    fn arguments(&mut self) -> ParserResult<Vec<Expr>> {
        let mut args: Vec<Expr> = Vec::new();

        args.push(self.expression()?);

        while self.matches(vec![TokenType::Comma]) {
            args.push(self.expression()?)
        }

        if args.len() > 250 {
            Err(ParserError::ArgumentLimitReached(self.position()))
        } else {
            Ok(args)
        }
    }

    /// call  -> primary ( "(" arguments? ")" )* ;
    fn call(&mut self) -> ParserResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(vec![TokenType::LeftParen]) {
                if self.matches(vec![TokenType::RightParen]) {
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        arguments: Vec::new(),
                        position: self.position(),
                    }
                } else {
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        arguments: self.arguments()?,
                        position: self.position(),
                    };
                    self.consume(
                        TokenType::RightParen,
                        "Expected a closing bracket ')' in call statement",
                    )?;
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// unary -> ( "!" | "-" ) unary | call ;
    fn unary(&mut self) -> ParserResult<Expr> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();
            let rhs = self.unary()?;

            Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs),
                position,
            })
        } else {
            self.call()
        }
    }

    /// factor -> unary ( ( "/" | "*" ) factor )* ;
    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();
            let rhs = self.factor()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                position,
            };
        }

        Ok(expr)
    }

    /// term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();
            let rhs = self.factor()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                position,
            };
        }

        Ok(expr)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;

        while self.matches(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();
            let rhs = self.term()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                position,
            };
        }

        Ok(expr)
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison)*
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();
            let rhs = self.comparison()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                position,
            };
        }

        Ok(expr)
    }

    ///! Breaks the parser "32 ;\n false ? 32 : 323;\n";
    /// ternary -> expression ( "?" ternary ":" ternary )?
    fn ternary(&mut self) -> ParserResult<Expr> {
        let mut expr = self.equality()?;

        if self.matches(vec![TokenType::QuestionMark]) {
            let result_1 = self.ternary()?;

            if self.matches(vec![TokenType::Colon]) {
                let result_2 = Box::new(self.ternary()?);
                expr = Expr::Ternary {
                    condition: Box::new(expr),
                    result_1: Box::new(result_1),
                    result_2,
                    position: self.position(),
                };

                return Ok(expr);
            } else {
                return Err(ParserError::Expected {
                    found: self.peek().unwrap().token_type,
                    msg: "Expected colon after the second expression in a ternary expression."
                        .to_string(),
                    position: self.position(),
                });
            }
        }

        Ok(expr)
    }

    /// logical_and -> ternary ("and" ternary)* ;
    fn logical_and(&mut self) -> ParserResult<Expr> {
        let mut expr = self.ternary()?;

        while self.matches(vec![TokenType::And]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(self.ternary()?),
                position,
            };
        }

        Ok(expr)
    }

    /// logical_or -> logic_and ("and" logic_and)* ;
    fn logical_or(&mut self) -> ParserResult<Expr> {
        let mut expr = self.logical_and()?;

        while self.matches(vec![TokenType::Or]) {
            let op = self.previous().unwrap();
            let position = op.position.clone();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(self.logical_and()?),
                position,
            };
        }

        Ok(expr)
    }

    /// assignment -> IDENTIFIER "=" assignment
    ///             | logical_or ;
    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.logical_or()?;

        match &expr {
            Expr::Identifier(name, _position) => {
                if self.matches(vec![TokenType::Equal]) {
                    let value = Box::new(self.assignment()?);

                    Ok(Expr::Assignment {
                        name: name.to_owned(),
                        value,
                        position: self.position(),
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
    fn expression(&mut self) -> ParserResult<Expr> {
        let expr = self.assignment()?;
        Ok(expr)
    }

    /// exprStatement  -> expression ";";
    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        let stmt = Stmt::ExprStmt(self.expression()?);
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(stmt)
    }

    /// printStatement -> "print" expression ";";
    fn print_statement(&mut self) -> ParserResult<Stmt> {
        let stmt = Stmt::PrintStmt(self.expression()?);
        self.consume(TokenType::Semicolon, "Expected ';' after 'print' statement")?;

        Ok(stmt)
    }

    /// block -> "{" declaration* "}" ;
    fn block(&mut self) -> ParserResult<Stmt> {
        let mut declarations: Vec<Stmt> = Vec::new();

        while !self.matches(vec![TokenType::RightBrace]) {
            declarations.push(self.declaration()?)
        }

        Ok(Stmt::Block(declarations))
    }

    fn if_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' before condition in an 'if' statement",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition in an 'if' statement",
        )?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.matches(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
            position: self.position(),
        })
    }

    /// whileStmt -> "while" "(" expression ")" statement;
    fn while_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' before condition in while loop",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition in 'while' loop",
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::WhileStmt {
            condition,
            body,
            position: self.position(),
        })
    }

    /// forStmt -> "for" "(" ( varDecl | exprStmt | ";")
    ///            expression? ";"
    ///            expression? ")" statement;
    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' before condition in 'for' loop",
        )?;

        // ( varDecl | exprStmt | ";") ;
        let initializer: Option<Stmt>;
        if self.matches(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.matches(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        // expression? ";" ;
        let condition: Option<Expr>;
        if self.matches(vec![TokenType::Semicolon]) {
            condition = None;
        } else {
            condition = Some(self.expression()?);
            self.consume(
                TokenType::Semicolon,
                "Expected ';' after condition in for loop",
            )?;
        }

        // expression? ")" ;
        let increment: Option<Expr>;
        if self.matches(vec![TokenType::RightParen]) {
            increment = None;
        } else {
            increment = Some(self.expression()?);
            self.consume(
                TokenType::RightParen,
                "Expected ')' after increment in for loop",
            )?;
        }

        let mut block_declarations: Vec<Stmt> = Vec::new();

        if let Some(stmt) = initializer {
            block_declarations.push(stmt)
        }

        // Parse for loop
        let condition = if condition.is_none() {
            Expr::Value {
                value: LoxValue::Boolean(true),
                position: self.position(),
            }
        } else {
            condition.unwrap()
        };

        let while_body: Stmt = if let Some(increment) = increment {
            Stmt::Block(vec![self.statement()?, Stmt::ExprStmt(increment)])
        } else {
            Stmt::Block(vec![self.statement()?])
        };

        let while_stmt = Stmt::WhileStmt {
            condition,
            body: Box::new(while_body),
            position: self.position(),
        };

        block_declarations.push(while_stmt);

        Ok(Stmt::Block(block_declarations))
    }

    /// breakStmt  -> "break" ";" ;
    fn break_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(
            TokenType::Semicolon,
            "Expected ';' at the end of a 'break' statement",
        )?;
        Ok(Stmt::BreakStmt(self.position()))
    }

    /// breakStmt  -> "continue" ";" ;
    fn continue_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(
            TokenType::Semicolon,
            "Expected ';' at the end of a 'continue' statement",
        )?;
        Ok(Stmt::ContinueStmt(self.position()))
    }

    /// returnStmt  -> "return" expression? ";" ;
    fn return_statement(&mut self) -> ParserResult<Stmt> {
        let position = self.position();

        if self.matches(vec![TokenType::Semicolon]) {
            Ok(Stmt::ReturnStmt {
                expr: None,
                position,
            })
        } else {
            let expr = Some(self.expression()?);
            self.consume(
                TokenType::Semicolon,
                "Expected ';' at the end of a 'return' statement",
            )?;
            Ok(Stmt::ReturnStmt { expr, position })
        }
    }

    /// statement -> exprStmt
    ///           |  printStmt
    ///           |  block   
    ///           |  ifStmt
    ///           |  whileStmt
    ///           |  forStmt   
    ///           |  breakStmt
    ///           |  continueStmt
    ///           |  returntmt     ;
    pub fn statement(&mut self) -> ParserResult<Stmt> {
        let stmt = if self.matches(vec![TokenType::Print]) {
            self.print_statement()?
        } else if self.matches(vec![TokenType::LeftBrace]) {
            self.block()?
        } else if self.matches(vec![TokenType::If]) {
            self.if_statement()?
        } else if self.matches(vec![TokenType::While]) {
            self.while_statement()?
        } else if self.matches(vec![TokenType::For]) {
            self.for_statement()?
        } else if self.matches(vec![TokenType::Break]) {
            self.break_statement()?
        } else if self.matches(vec![TokenType::Continue]) {
            self.continue_statement()?
        } else if self.matches(vec![TokenType::Return]) {
            self.return_statement()?
        } else {
            self.expression_statement()?
        };

        Ok(stmt)
    }

    /// varDeclaration -> "var" IDENTIFIER ("=" expression)?;
    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let ident_token = self.advance().unwrap();

        let name = if let Some(LoxValue::Identifier(s)) = ident_token.literal {
            s
        } else {
            return Err(ParserError::Expected {
                found: ident_token.token_type,
                msg: "Expected identifier after 'var' in variable declaration.".to_string(),
                position: self.position(),
            });
        };

        let initializer = if self.matches(vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Value {
                value: LoxValue::Nil,
                position: self.position(),
            }
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' at the end of a variable declaration",
        )?;

        Ok(Stmt::Var {
            name,
            initializer,
            postion: self.position(),
        })
    }

    /// IDENTIFIER "(" arguments? ")" block ;
    fn function(&mut self) -> ParserResult<Stmt> {
        let name = if let LoxValue::Identifier(ident) = self.advance().unwrap().literal.unwrap() {
            ident
        } else {
            return Err(ParserError::Expected {
                found: self.prev_token_type(),
                msg: "Expected an identifier in the function declaration.".to_string(),
                position: self.position(),
            });
        };

        self.consume(
            TokenType::LeftParen,
            "Expected '(' before parameters in function declaration",
        )?;

        let mut params: Vec<String> = Vec::new();
        if !self.matches(vec![TokenType::RightParen]) {
            for arg in self.arguments()? {
                match arg {
                    Expr::Identifier(ident, _position) => params.push(ident),
                    _ => {
                        return Err(ParserError::Expected {
                            found: TokenType::Nil,
                            msg: "Parameters in function declaration must be identifiers"
                                .to_string(),
                            position: self.position(),
                        });
                    }
                }
            }
            self.consume(
                TokenType::RightParen,
                "Expected ')' after parameters in a function declaration",
            )?;
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' at the beginning of the body of a function",
        )?;

        let body = Box::new(self.block()?);

        let fun_declaration = FunDecl { name, params, body };

        Ok(Stmt::FunStmt {
            fun_declaration,
            position: self.position(),
        })
    }

    /// "fun" function ;
    fn fun_declaration(&mut self) -> ParserResult<Stmt> {
        self.function()
    }

    /// declaration -> varDeclaration
    ///              | funDeclaration
    ///              | statement      ;
    fn declaration(&mut self) -> ParserResult<Stmt> {
        if self.matches(vec![TokenType::Var]) {
            self.var_declaration()
        } else if self.matches(vec![TokenType::Fun]) {
            self.fun_declaration()
        } else {
            self.statement()
        }
    }

    /// program -> declaration* EOF
    pub fn program(&mut self) -> ParserResult<Vec<Stmt>> {
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

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(ParserError::Bundle(errors))
        }
    }

    pub fn parse_str(src: &str) -> LoxResult<Vec<Stmt>> {
        let tokens = Scanner::tokens_from_str(src, false);
        Parser::new(tokens)
            .program()
            .map_err(|e| LoxError::Parser(e))
    }
}

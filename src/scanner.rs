use std::collections::HashMap;

use crate::lox_literal::LoxLiteral;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    pub source: Vec<u8>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("or", TokenType::Or),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("nil", TokenType::Nil),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("class", TokenType::Class),
                ("return", TokenType::Return),
                ("print", TokenType::Print),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("var", TokenType::Var),
            ]),
        }
    }

    // Checks self.current is at the end of source.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Yields value at self.current and increment self.current.
    fn advance(&mut self) -> u8 {
        let res = self.source[self.current];
        self.current += 1;
        res
    }

    // Looks ahead at value at self.current
    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    // Looks ahead twiec at value at self.current + 1
    fn peek_twice(&self) -> u8 {
        self.source[self.current + 1]
    }

    // Checks if the byte at self.current is equal to expected.
    fn matches_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.peek() != expected {
            return false;
        };

        self.current += 1;
        true
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LoxLiteral>) {
        let tok = match literal {
            Some(_) => {
                // let lexeme = Some(self.get_curr_string());
                // TODO Readd Lexeme
                Token::new(self.line, token_type, literal)
            }
            None => Token::new(self.line, token_type, None),
        };

        self.tokens.push(tok)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    /// Gets the string between self.start and self.current.
    fn get_curr_string(&self) -> String {
        self.source[self.start..self.current]
            .iter()
            .cloned()
            .map(|b| b as char)
            .collect()
    }

    fn scan_comment(&mut self) {
        // While there is a comment keep on calling advance
        if self.matches_next(b'/') {
            while !self.is_at_end() && (self.peek() != b'\n') {
                self.advance();
            }
        } else if self.matches_next(b'*') {
            while !self.is_at_end() {
                if self.peek() == b'\n' {
                    self.line += 1
                }
                if self.peek() == b'*' && self.peek_twice() == b'/' {
                    self.current += 2;
                    break;
                }

                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash)
        }
    }

    fn scan_string(&mut self) {
        while !self.matches_next(b'"') && !self.is_at_end() {
            if self.matches_next(b'\n') {
                self.line += 1;
            }
            self.advance();
        }

        self.add_token_with_literal(
            TokenType::String,
            Some(LoxLiteral::String(self.get_curr_string())),
        )
    }

    fn scan_number(&mut self) {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.is_at_end() && self.matches_next(b'.') && self.peek().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number = self.get_curr_string().parse::<f64>().unwrap();
        self.add_token_with_literal(TokenType::Number, Some(LoxLiteral::Number(number)));
    }

    fn scan_identifier(&mut self) {
        while !self.is_at_end() && self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let literal = self.get_curr_string();

        if let Some(token_type) = self.keywords.get(literal.as_str()) {
            let token_type = token_type.clone();

            match token_type {
                TokenType::True => {
                    self.add_token_with_literal(token_type, Some(LoxLiteral::Boolean(true)))
                }
                TokenType::False => {
                    self.add_token_with_literal(token_type, Some(LoxLiteral::Boolean(false)))
                }
                _ => self.add_token(token_type),
            }
        } else {
            self.add_token_with_literal(
                TokenType::Identifier,
                Some(LoxLiteral::Identifier(literal)),
            )
        }
    }

    // @desc Calls advance and matches to handle the token.
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'?' => self.add_token(TokenType::QuestionMark),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b'*' => self.add_token(TokenType::Star),
            b';' => self.add_token(TokenType::Semicolon),
            b':' => self.add_token(TokenType::Colon),

            b'!' => {
                let token_type = if self.matches_next(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            b'=' => {
                let token_type = if self.matches_next(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            b'<' => {
                let token_type = if self.matches_next(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            b'>' => {
                let token_type = if self.matches_next(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            // TODO
            b'/' => self.scan_comment(),
            b'"' => self.scan_string(),
            b if b.is_ascii_digit() => self.scan_number(),
            b if b.is_ascii_alphanumeric() => self.scan_identifier(),

            b'\n' => self.line += 1,
            b' ' | b'\r' | b'\t' => (),
            _ => println!("Invalid character {}", self.line),
        }
    }

    // @desc Call scan_token till it's done with self.source.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof);

        &self.tokens
    }

    pub fn print_tokens(&self) {
        println!("\nScanned Tokens: ");
        println!("============================================================================================");
        let mut line_no = 0;

        for (index, token) in self.tokens.iter().enumerate() {
            if token.line != line_no {
                line_no = token.line;
                println!("\nLine {}", line_no);
                println!("==================");
            }
            println!("{} ->  {:?}", index, token);
        }
        println!("\n============================================================================================\n");
    }

    pub fn tokens_from_str(source: &str, display: bool) -> Vec<Token> {
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        scanner.scan_tokens();

        if display {
            scanner.print_tokens();
        }

        scanner.tokens
    }

    pub fn tokens_from_bytes(source: &[u8]) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_vec());
        scanner.scan_tokens();
        scanner.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_number_of_tokens() {
        let tokens = Scanner::tokens_from_str("var \n x = if (5 * 7) \n 8 else 9.7823", false);

        // assert_eq!(res.len(), 4, "Scans correct number of tokens")
    }
}

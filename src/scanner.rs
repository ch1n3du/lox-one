use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use std::collections::HashMap;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    fn new(source: Vec<u8>) -> Scanner {
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

    // Scanner helper functions

    // @desc Checks self.current is at the end of source.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // @desc Yields value at self.current and increment self.current.
    fn advance(&mut self) -> u8 {
        let res = self.source[self.current];
        self.current += 1;
        res
    }

    // @desc Looks ahead at value at self.current
    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    // @desc Looks ahead twiec at value at self.current + 1
    fn peek_twice(&self) -> u8 {
        self.source[self.current + 1]
    }

    // @desc Checks if the byte at self.current is equal to expected.
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

    // @desc Takes a TokenType and Literal, creates a new Token and pushses it to self.tokens.
    fn add_token(&mut self, ty: TokenType, literal: Option<Literal>) {
        let tok = match literal {
            Some(_) => {
                let lexeme = Some(self.source[self.start..self.current].to_vec());
                Token::new(ty, lexeme, literal, self.line)
            }
            _ => Token::new(ty, None, None, self.line),
        };

        self.tokens.push(tok)
    }

    fn add_next_token_if_matches(&mut self, expected: u8, token_1: TokenType, token_2: TokenType) {
        if self.matches_next(expected) {
            self.add_token(TokenType::BangEqual, None)
        } else {
            self.add_token(TokenType::Bang, None)
        }
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
            self.add_token(TokenType::Slash, None)
        }
    }

    fn scan_string(&mut self) {
        while !self.matches_next(b'"') && !self.is_at_end() {
            if self.matches_next(b'\n') {
                self.line += 1;
            }
            self.advance();
        }
        self.add_token(TokenType::String, Some(Literal::Todo))
    }

    fn scan_number(&mut self) {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        if  self.is_at_end() && self.matches_next(b'.') && self.peek().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.add_token(TokenType::Number, Some(Literal::Todo));
    }

    fn scan_identifier_(&mut self) -> String {
        while !self.is_at_end() && self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let identifier: String = self.source[self.start..self.current]
            .iter()
            .cloned()
            .map(|c| c as char)
            .collect::<String>();

        identifier
    }

    fn set_identifier(&mut self, lexeme: String) {
        match self.keywords.get(lexeme.as_str()) {
            Some(ty) => self.tokens.push(Token::new(
                ty.clone(),
                Some(lexeme.as_bytes().to_vec()),
                None,
                self.line,
            )),
            None => self.add_token(TokenType::Identifier, Some(Literal::Todo)),
        }
    }

    fn scan_identifier(&mut self) {
        let identifier = self.scan_identifier_();
        self.set_identifier(identifier);
    }

    // @desc Calls advance and matches to handle the token.
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'r' => self.add_token(TokenType::Star, None),

            b'!' => self.add_next_token_if_matches(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.add_next_token_if_matches(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.add_next_token_if_matches(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => {
                self.add_next_token_if_matches(b'=', TokenType::GreaterEqual, TokenType::Greater)
            }

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
    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof, None);

        &self.tokens
    }

    pub fn print_tokens(&self) {
        println!("\nScanned Tokens: ");
        println!("============================================================================================\n");
        let mut line_no = 0;

        for token in &self.tokens {
            if token.line != line_no {
                line_no = token.line;
                println!("\nLine {}", line_no);
                println!("==================");
            }
            println!("->  {:?}", token);
        }
        println!("\n============================================================================================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tokens_from_string(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        scanner.scan_tokens();
        scanner.print_tokens();

        scanner.tokens
    }

    fn tokens_from_bytes(source: &[u8]) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_vec());
        scanner.scan_tokens();
        scanner.tokens
    }

    #[test]
    fn correct_number_of_tokens() {
        let res = tokens_from_string("var \n x = if (5 > 7) \n 8 else 9.7823");

        // assert_eq!(res.len(), 4, "Scans correct number of tokens")
    }
}

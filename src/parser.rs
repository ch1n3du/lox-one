/*
"Grammar, which knows how to control even kings."
                                    - Moliere
*/

use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, start: 0, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Gets Token at `current` next to be consumed.
    fn peek(&self) -> Option<Token> {
        if self.is_at_end() { return None }
        Some(self.tokens[self.current])
    }

    fn previous(&self) -> Option<Token> {
        if self.current == 0 { return None }
        Some(self.tokens[self.current-1])
    }

    /// Checks if next Token matches any of the Tokens in tokens
    fn next_matches_any(&self, tokens: Vec<Token> ) -> bool {
        let next_ = self.peek();
        if next_.is_none() { return false }
        let next = next_.unwrap();

        tokens.iter().any(|t| t.clone() == next)
    }
}
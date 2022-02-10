use crate::lexer::Token;

use super::visitors::Expression;

pub struct Parser {
    pub lexer: Vec<Token>,
    current_token: usize,
}

impl Parser {
    pub fn new(lexer: Vec<Token>) -> Self {
        Self {
            lexer,
            current_token: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        if let Ok(ret) = self.or() {
            Some(ret)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.is_at_the_end() {
            return None;
        }

        Some(&self.lexer[self.current_token])
    }

    pub fn expect(&mut self, token: &Token) -> Option<&Token> {
        if self.is_at_the_end() {
            return None;
        }

        if self.peek().unwrap() == token {
            Some(self.advance()?)
        } else {
            None
        }
    }

    pub fn expect_tokens(&mut self, tokens: &[Token]) -> Option<&Token> {
        if self.is_at_the_end() {
            return None;
        }

        for token in tokens {
            if self.peek().unwrap() == token {
                return Some(self.advance()?);
            }
        }

        return None;
    }

    pub fn is_at_the_end(&self) -> bool {
        self.current_token >= self.lexer.len() || self.lexer[self.current_token] == Token::EndOfFile
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_the_end() {
            self.current_token += 1;
        }

        self.previous()
    }

    pub fn previous(&self) -> Option<&Token> {
        Some(&self.lexer[self.current_token - 1])
    }

    pub fn consume(&mut self, token: &Token, error_message: &str) -> bool {
        if !self.check(token) {
            println!("Error: {}", error_message);
            return false;
        }

        self.advance();

        true
    }

    fn check(&self, token: &Token) -> bool {
        !self.is_at_the_end() && self.lexer[self.current_token] == *token
    }
}

use crate::lexer::{LogosToken, Token};

use super::visitors::Statement;

pub struct Parser {
    pub lexer: Vec<Token>,
    pub(crate) module_path: String,
    current_token: usize,
}

impl Parser {
    /// # Arguments
    /// - lexer: vec of LogosToken (usually collected from the lexer)
    /// - module_path: String path of the current module
    ///
    /// The module path arguments is used by the parser for import statements
    /// nodes of the AST.
    pub fn new(lexer: Vec<Token>, module_path: &str) -> Self {
        Self {
            lexer,
            module_path: module_path.to_string(),
            current_token: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Statement>> {
        let mut stmts: Vec<Statement> = Vec::new();

        while !self.is_at_the_end() {
            if let Ok(ret) = self.parse_import_statement() {
                stmts.push(ret);
            } else {
                return None;
            }
        }

        Some(stmts)
    }

    pub fn peek(&self) -> Option<&LogosToken> {
        if self.is_at_the_end() {
            return None;
        }

        Some(&self.lexer[self.current_token].logos_tk)
    }

    pub fn peek_token_with_info_debug(&self) -> &Token {
        if self.is_at_the_end() {
            &self.lexer[self.current_token - 1]
        } else {
            &self.lexer[self.current_token]
        }
    }

    pub fn get_current_token_postion(&self) -> (usize, usize) {
        let tk = self.peek_token_with_info_debug();

        (tk.line_number, tk.column_number)
    }

    pub fn expect(&mut self, token: &LogosToken) -> Option<&LogosToken> {
        if self.is_at_the_end() {
            return None;
        }

        if self.peek().unwrap() == token {
            Some(self.advance()?)
        } else {
            None
        }
    }

    pub fn expect_tokens(&mut self, tokens: &[LogosToken]) -> Option<&LogosToken> {
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
        self.current_token >= self.lexer.len()
            || self.lexer[self.current_token].logos_tk == LogosToken::EndOfFile
    }

    pub fn advance(&mut self) -> Option<&LogosToken> {
        if !self.is_at_the_end() {
            self.current_token += 1;
        }

        self.previous()
    }

    pub fn previous(&self) -> Option<&LogosToken> {
        Some(&self.lexer[self.current_token - 1].logos_tk)
    }

    pub fn consume(&mut self, token: &LogosToken, error_message: &str) -> Option<&LogosToken> {
        if !self.check(token) {
            if !self.is_at_the_end() {
                let current_tk = &self.lexer[self.current_token];
                println!(
                    "{}:{}:{} Error: {}",
                    self.module_path,
                    current_tk.line_number,
                    current_tk.column_number,
                    error_message
                );
            } else {
                println!("{}: Error: {}", self.module_path, error_message);
            }

            return None;
        }

        self.advance()
    }

    pub fn put_error_at_current_token(&self, error_message: &str) {
        let current_tk = if !self.is_at_the_end() {
            &self.lexer[self.current_token]
        } else {
            &self.lexer[self.current_token - 1]
        };

        println!(
            "{}:{}:{} Error: {}",
            self.module_path, current_tk.line_number, current_tk.column_number, error_message
        );
    }

    pub fn match_expr(&mut self, token_types: &[LogosToken]) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn check(&self, token: &LogosToken) -> bool {
        !self.is_at_the_end() && self.lexer[self.current_token].logos_tk == *token
    }
}

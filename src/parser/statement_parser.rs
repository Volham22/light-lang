use crate::{lexer::Token, type_system::type_check::ValueType};

use super::{
    parser::Parser,
    visitors::{Expression, Literal, Statement, VariableAssignment, VariableDeclaration},
};

impl Parser {
    pub fn parse_declaration_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::Let]) {
            let identifier = match self.consume(
                &Token::Identifier(String::new()),
                "Expected identifier after Let",
            ) {
                Some(Token::Identifier(name)) => name.clone(),
                _ => {
                    return Err(());
                }
            };

            if let None = self.consume(&Token::Colon, "Expected ':' after identifier.") {
                return Err(());
            }

            let variable_type = match self.consume(
                &Token::Type(ValueType::Number),
                "Expected valid typename after ':'.",
            ) {
                Some(Token::Type(t)) => *t,
                _ => {
                    return Err(());
                },
            };

            if let None = self.consume(&Token::Equal, "Expected '=' after typename.") {
                return Err(());
            }

            let init_expr = self.or()?;

            if let None = self.consume(&Token::Semicolon, "Expected ';' after <init_expr>.") {
                return Err(());
            }

            return Ok(Statement::VariableDeclaration(VariableDeclaration {
                identifier,
                variable_type,
                init_expr,
            }));
        }

        self.parse_expression_statement()
    }

    pub fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        let expr = self.or()?;

        if self.match_expr(&[Token::Equal]) {
            let rhs = self.or()?;

            if let None = self.consume(&Token::Semicolon, "Expected ';' after assigment.") {
                return Err(());
            }

            if let Expression::Literal(Literal::Identifier(identifier)) = expr {
                return Ok(Statement::VariableAssignment(VariableAssignment {
                    identifier,
                    new_value: rhs,
                }));
            } else {
                println!("Error: left side of assignment must be an lvalue.");
                return Err(());
            }
        }

        if let None = self.consume(&Token::Semicolon, "Expected ';' after <expression>") {
            return Err(());
        }

        Ok(Statement::Expression(expr))
    }
}

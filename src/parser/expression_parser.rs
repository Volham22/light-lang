use crate::lexer::Token;

use super::{parser::Parser, visitors::Expression};

impl Parser {
    pub fn or(&mut self) -> Result<Expression, ()> {
        let mut left = self.and()?;

        while let Some(Token::Or) = self.expect(&Token::Or) {
            let right = self.and()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expression, ()> {
        let mut left = self.equality()?;

        while let Some(Token::And) = self.expect(&Token::And) {
            let right = self.equality()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut left = self.comp()?;

        loop {
            match self.expect_tokens(&[Token::Equality, Token::NegEquality]) {
                Some(Token::Equal) => {
                    let right = self.equality()?;
                    left = Expression::Equal(Box::new(left), Box::new(right));
                }
                Some(Token::NegEquality) => {
                    let right = self.equality()?;
                    left = Expression::NotEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn comp(&mut self) -> Result<Expression, ()> {
        let mut left = self.term()?;

        loop {
            match self.expect_tokens(&[Token::Less, Token::More, Token::LessEqual, Token::MoreEqual]) {
                Some(Token::Less) => {
                    let right = self.term()?;
                    left = Expression::Less(Box::new(left), Box::new(right));
                }
                Some(Token::More) => {
                    let right = self.term()?;
                    left = Expression::More(Box::new(left), Box::new(right));
                }
                Some(Token::LessEqual) => {
                    let right = self.term()?;
                    left = Expression::LessEqual(Box::new(left), Box::new(right));
                }
                Some(Token::MoreEqual) => {
                    let right = self.term()?;
                    left = Expression::MoreEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, ()> {
        let mut left = self.factor()?;

        loop {
            match self.expect_tokens(&[Token::Plus, Token::Minus]) {
                Some(Token::Plus) => {
                    let right = self.factor()?;
                    left = Expression::Plus(Box::new(left), Box::new(right));
                }
                Some(Token::Minus) => {
                    let right = self.factor()?;
                    left = Expression::Minus(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, ()> {
        let mut left = self.unary()?;

        loop {
            match self.expect_tokens(&[Token::Multiply, Token::Divide, Token::Modulo]) {
                Some(Token::Multiply) => {
                    let right = self.unary()?;
                    left = Expression::Multiply(Box::new(left), Box::new(right));
                }
                Some(Token::Divide) => {
                    let right = self.unary()?;
                    left = Expression::Divide(Box::new(left), Box::new(right));
                }
                Some(Token::Modulo) => {
                    let right = self.unary()?;
                    left = Expression::Modulo(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        let expr = self.primary()?;

        match self.expect_tokens(&[Token::Minus, Token::Not]) {
            Some(Token::Minus) => Ok(Expression::Negate(Box::new(expr))),
            Some(Token::Not) => Ok(Expression::Not(Box::new(expr))),
            _ => Ok(expr),
        }
    }

    fn primary(&mut self) -> Result<Expression, ()> {
        let token = self.advance();

        match token {
            Some(Token::True) => Ok(Expression::Bool(true)),
            Some(Token::False) => Ok(Expression::Bool(false)),
            Some(Token::Number(value)) => Ok(Expression::Number(*value)),
            Some(Token::Real(value)) => Ok(Expression::Real(*value)),
            Some(Token::Identifier(value)) => Ok(Expression::Identifier(value.to_string())),
            _ => {
                if let Some(t) = token {
                    println!("Error: Unexpected {:?}", t);
                } else {
                    println!("Error: Unexpected EOF");
                }

                Err(())
            },
        }
    }
}

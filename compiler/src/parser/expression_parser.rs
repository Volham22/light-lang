use crate::lexer::Token;

use super::{
    parser::Parser,
    visitors::{
        AddressOf, ArrayAccess, Binary, BinaryLogic, Call, DeReference, Expression, Group,
        Identifier, Literal, MemberAccess, StructLiteral, Unary,
    },
};

impl Parser {
    pub fn or(&mut self) -> Result<Expression, ()> {
        let mut left = self.and()?;

        while let Some(Token::Or) = self.expect(&Token::Or) {
            let right = self.and()?;
            left = Expression::BinaryLogic(BinaryLogic::Or(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expression, ()> {
        let mut left = self.equality()?;

        while let Some(Token::And) = self.expect(&Token::And) {
            let right = self.equality()?;
            left = Expression::BinaryLogic(BinaryLogic::And(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut left = self.comp()?;

        loop {
            match self.expect_tokens(&[Token::Equality, Token::NegEquality]) {
                Some(Token::Equality) => {
                    let right = self.equality()?;
                    left = Expression::BinaryLogic(BinaryLogic::Equal(
                        Box::new(left),
                        Box::new(right),
                    ));
                }
                Some(Token::NegEquality) => {
                    let right = self.equality()?;
                    left = Expression::BinaryLogic(BinaryLogic::NotEqual(
                        Box::new(left),
                        Box::new(right),
                    ));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn comp(&mut self) -> Result<Expression, ()> {
        let mut left = self.term()?;

        loop {
            match self.expect_tokens(&[
                Token::Less,
                Token::More,
                Token::LessEqual,
                Token::MoreEqual,
            ]) {
                Some(Token::Less) => {
                    let right = self.term()?;
                    left =
                        Expression::BinaryLogic(BinaryLogic::Less(Box::new(left), Box::new(right)));
                }
                Some(Token::More) => {
                    let right = self.term()?;
                    left =
                        Expression::BinaryLogic(BinaryLogic::More(Box::new(left), Box::new(right)));
                }
                Some(Token::LessEqual) => {
                    let right = self.term()?;
                    left = Expression::BinaryLogic(BinaryLogic::LessEqual(
                        Box::new(left),
                        Box::new(right),
                    ));
                }
                Some(Token::MoreEqual) => {
                    let right = self.term()?;
                    left = Expression::BinaryLogic(BinaryLogic::MoreEqual(
                        Box::new(left),
                        Box::new(right),
                    ));
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
                    left = Expression::Binary(Binary::Plus(Box::new(left), Box::new(right)));
                }
                Some(Token::Minus) => {
                    let right = self.factor()?;
                    left = Expression::Binary(Binary::Minus(Box::new(left), Box::new(right)));
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
                    left = Expression::Binary(Binary::Multiply(Box::new(left), Box::new(right)));
                }
                Some(Token::Divide) => {
                    let right = self.unary()?;
                    left = Expression::Binary(Binary::Divide(Box::new(left), Box::new(right)));
                }
                Some(Token::Modulo) => {
                    let right = self.unary()?;
                    left = Expression::Binary(Binary::Modulo(Box::new(left), Box::new(right)));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        match self.expect_tokens(&[Token::Minus, Token::Not]) {
            Some(Token::Minus) => Ok(Expression::Unary(Unary::Negate(Box::new(self.call()?)))),
            Some(Token::Not) => Ok(Expression::Unary(Unary::Not(Box::new(self.call()?)))),
            _ => Ok(self.call()?),
        }
    }

    fn call(&mut self) -> Result<Expression, ()> {
        let primary_expr = self.primary()?;

        if self.match_expr(&[Token::LeftParenthesis]) {
            let mut args: Vec<Expression> = Vec::new();

            loop {
                if self.check(&Token::RightParenthesis) {
                    break;
                }

                args.push(self.or()?);

                if !self.match_expr(&[Token::Comma]) {
                    break;
                }
            }

            if let None = self.consume(&Token::RightParenthesis, "Unclosed '(' in function call.") {
                return Err(());
            }

            let identifier = match primary_expr {
                Expression::Literal(Literal::Identifier(n)) => n,
                _ => {
                    println!("Error: Expected identifier before function call.");
                    return Err(());
                }
            };

            return Ok(Expression::Call(Call {
                name: identifier.name,
                ty: None,
                args: if !args.is_empty() { Some(args) } else { None },
            }));
        }

        if self.match_expr(&[Token::Dot]) {
            let rhs = if let Expression::Literal(Literal::Identifier(id)) = self.or()? {
                id.name
            } else {
                println!("Expected identifier after Dot member access.");
                return Err(());
            };

            return Ok(Expression::MemberAccess(MemberAccess {
                object: Box::new(primary_expr),
                member: rhs.to_string(),
                ty: None,
            }));
        }

        if self.match_expr(&[Token::LeftBracket]) {
            let matched_token = self.previous().unwrap();

            match matched_token {
                Token::LeftBracket => {
                    let index = self.or()?;

                    if let None =
                        self.consume(&Token::RightBracket, "Unclosed ']' in array access.")
                    {
                        return Err(());
                    }

                    return Ok(Expression::ArrayAccess(ArrayAccess {
                        ty: None,
                        identifier: Box::new(primary_expr),
                        is_lvalue: false,
                        index: Box::new(index.clone()),
                    }));
                }
                _ => unreachable!(),
            }
        }

        // TODO: Support namespace
        // if self.match_expr(&[Token::DoubleColon]) {
        //     let rhs = self.or()?;

        //     return Ok(Expression::ModuleAccess(ModuleAccess {
        //         left: Box::new(primary_expr),
        //         right: Box::new(rhs),
        //     }));
        // }

        Ok(primary_expr)
    }

    fn primary(&mut self) -> Result<Expression, ()> {
        let token = self.advance();

        match token {
            Some(Token::True) => Ok(Expression::Literal(Literal::Bool(true))),
            Some(Token::False) => Ok(Expression::Literal(Literal::Bool(false))),
            Some(Token::Number(value)) => Ok(Expression::Literal(Literal::Number(*value))),
            Some(Token::Real(value)) => Ok(Expression::Literal(Literal::Real(*value))),
            Some(Token::Quote(s)) => Ok(Expression::Literal(Literal::StringLiteral(s.clone()))),
            Some(Token::Null) => Ok(Expression::Null),
            Some(Token::AddressOf) => {
                let identifier = self.or()?;
                Ok(Expression::AddressOf(AddressOf {
                    identifier: Box::new(identifier),
                    ty: None,
                }))
            }
            Some(Token::Dereference) => {
                let identifier = self.or()?;

                Ok(Expression::DeReference(DeReference {
                    identifier: Box::new(identifier),
                    is_lvalue: false,
                    ty: None,
                }))
            }
            Some(Token::Identifier(value)) => {
                let name = value.clone(); // Copy the literal's name to avoid borrow checker errors

                Ok(Expression::Literal(Literal::Identifier(Identifier {
                    name,
                    is_lvalue: false,
                    ty: None,
                })))
            }
            Some(Token::LeftParenthesis) => {
                let inner_expr = self.or()?;

                if let None = self.consume(&Token::RightParenthesis, "Unclosed parenthesis.") {
                    return Err(());
                }

                Ok(Expression::Group(Group {
                    inner_expression: Box::new(inner_expr),
                }))
            }
            Some(Token::Struct) => {
                let type_name = if let Some(Token::Identifier(name)) = self.consume(
                    &Token::Identifier(String::new()),
                    "Expected type name after 'struct' keyword in expression",
                ) {
                    name.to_string()
                } else {
                    return Err(());
                };

                if let None =
                    self.consume(&Token::LeftBrace, "Expected '{' in struct initialization")
                {
                    return Err(());
                }

                let mut expressions: Vec<Expression> = Vec::new();
                loop {
                    // A right brace mark the end of the expression list.
                    if self.check(&Token::RightBrace) {
                        break;
                    }
                    expressions.push(self.or()?);

                    // Expect a comma

                    if !self.match_expr(&[Token::Comma]) {
                        break;
                    }
                }

                if let None =
                    self.consume(&&Token::RightBrace, "Unclosed '}' in struct initialization")
                {
                    return Err(());
                }

                Ok(Expression::Literal(Literal::StructLiteral(StructLiteral {
                    literal_type: None,
                    type_name,
                    expressions,
                })))
            }
            _ => {
                if let Some(t) = token {
                    println!("Error: Unexpected {:?}", t);
                } else {
                    println!("Error: Unexpected EOF");
                }

                Err(())
            }
        }
    }
}

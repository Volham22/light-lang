use crate::lexer::LogosToken;

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

        while let Some(LogosToken::Or) = self.expect(&LogosToken::Or) {
            let right = self.and()?;
            left = Expression::BinaryLogic(BinaryLogic::Or(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expression, ()> {
        let mut left = self.equality()?;

        while let Some(LogosToken::And) = self.expect(&LogosToken::And) {
            let right = self.equality()?;
            left = Expression::BinaryLogic(BinaryLogic::And(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut left = self.comp()?;

        loop {
            match self.expect_tokens(&[LogosToken::Equality, LogosToken::NegEquality]) {
                Some(LogosToken::Equality) => {
                    let right = self.equality()?;
                    left = Expression::BinaryLogic(BinaryLogic::Equal(
                        Box::new(left),
                        Box::new(right),
                    ));
                }
                Some(LogosToken::NegEquality) => {
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
                LogosToken::Less,
                LogosToken::More,
                LogosToken::LessEqual,
                LogosToken::MoreEqual,
            ]) {
                Some(LogosToken::Less) => {
                    let right = self.term()?;
                    left =
                        Expression::BinaryLogic(BinaryLogic::Less(Box::new(left), Box::new(right)));
                }
                Some(LogosToken::More) => {
                    let right = self.term()?;
                    left =
                        Expression::BinaryLogic(BinaryLogic::More(Box::new(left), Box::new(right)));
                }
                Some(LogosToken::LessEqual) => {
                    let right = self.term()?;
                    left = Expression::BinaryLogic(BinaryLogic::LessEqual(
                        Box::new(left),
                        Box::new(right),
                    ));
                }
                Some(LogosToken::MoreEqual) => {
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
            match self.expect_tokens(&[LogosToken::Plus, LogosToken::Minus]) {
                Some(LogosToken::Plus) => {
                    let right = self.factor()?;
                    left = Expression::Binary(Binary::Plus(Box::new(left), Box::new(right)));
                }
                Some(LogosToken::Minus) => {
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
            match self.expect_tokens(&[
                LogosToken::Multiply,
                LogosToken::Divide,
                LogosToken::Modulo,
            ]) {
                Some(LogosToken::Multiply) => {
                    let right = self.unary()?;
                    left = Expression::Binary(Binary::Multiply(Box::new(left), Box::new(right)));
                }
                Some(LogosToken::Divide) => {
                    let right = self.unary()?;
                    left = Expression::Binary(Binary::Divide(Box::new(left), Box::new(right)));
                }
                Some(LogosToken::Modulo) => {
                    let right = self.unary()?;
                    left = Expression::Binary(Binary::Modulo(Box::new(left), Box::new(right)));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        match self.expect_tokens(&[LogosToken::Minus, LogosToken::Not]) {
            Some(LogosToken::Minus) => Ok(Expression::Unary(Unary::Negate(Box::new(self.call()?)))),
            Some(LogosToken::Not) => Ok(Expression::Unary(Unary::Not(Box::new(self.call()?)))),
            _ => Ok(self.call()?),
        }
    }

    fn call(&mut self) -> Result<Expression, ()> {
        let primary_expr = self.primary()?;

        if self.match_expr(&[LogosToken::LeftParenthesis]) {
            let mut args: Vec<Expression> = Vec::new();

            loop {
                if self.check(&LogosToken::RightParenthesis) {
                    break;
                }

                args.push(self.or()?);

                if !self.match_expr(&[LogosToken::Comma]) {
                    break;
                }
            }

            if let None = self.consume(
                &LogosToken::RightParenthesis,
                "Unclosed '(' in function call.",
            ) {
                return Err(());
            }

            let identifier = match primary_expr {
                Expression::Literal(Literal::Identifier(n)) => n,
                _ => {
                    self.put_error_at_current_token(
                        "Error: Expected identifier before function call.",
                    );
                    return Err(());
                }
            };

            return Ok(Expression::Call(Call {
                name: identifier.name,
                ty: None,
                args: if !args.is_empty() { Some(args) } else { None },
            }));
        }

        if self.match_expr(&[LogosToken::Dot]) {
            let rhs = if let Expression::Literal(Literal::Identifier(id)) = self.or()? {
                id.name
            } else {
                self.put_error_at_current_token("Expected identifier after Dot member access.");
                return Err(());
            };

            return Ok(Expression::MemberAccess(MemberAccess {
                object: Box::new(primary_expr),
                member: rhs.to_string(),
                ty: None,
            }));
        }

        if self.match_expr(&[LogosToken::LeftBracket]) {
            let matched_token = self.previous().unwrap();

            match matched_token {
                LogosToken::LeftBracket => {
                    let index = self.or()?;

                    if let None =
                        self.consume(&LogosToken::RightBracket, "Unclosed ']' in array access.")
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
        // if self.match_expr(&[LogosToken::DoubleColon]) {
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
            Some(LogosToken::True) => Ok(Expression::Literal(Literal::Bool(true))),
            Some(LogosToken::False) => Ok(Expression::Literal(Literal::Bool(false))),
            Some(LogosToken::Number(value)) => Ok(Expression::Literal(Literal::Number(*value))),
            Some(LogosToken::CharLiteral(value)) => Ok(Expression::Literal(Literal::Char(*value))),
            Some(LogosToken::Real(value)) => Ok(Expression::Literal(Literal::Real(*value))),
            Some(LogosToken::Quote(s)) => {
                Ok(Expression::Literal(Literal::StringLiteral(s.clone())))
            }
            Some(LogosToken::Null) => Ok(Expression::Null),
            Some(LogosToken::AddressOf) => {
                let identifier = self.or()?;
                Ok(Expression::AddressOf(AddressOf {
                    identifier: Box::new(identifier),
                    ty: None,
                }))
            }
            Some(LogosToken::Dereference) => {
                let identifier = self.or()?;

                Ok(Expression::DeReference(DeReference {
                    identifier: Box::new(identifier),
                    is_lvalue: false,
                    ty: None,
                }))
            }
            Some(LogosToken::Identifier(value)) => {
                let name = value.clone(); // Copy the literal's name to avoid borrow checker errors

                Ok(Expression::Literal(Literal::Identifier(Identifier {
                    name,
                    is_lvalue: false,
                    ty: None,
                })))
            }
            Some(LogosToken::LeftParenthesis) => {
                let inner_expr = self.or()?;

                if let None = self.consume(&LogosToken::RightParenthesis, "Unclosed parenthesis.") {
                    return Err(());
                }

                Ok(Expression::Group(Group {
                    inner_expression: Box::new(inner_expr),
                }))
            }
            Some(LogosToken::Struct) => {
                let type_name = if let Some(LogosToken::Identifier(name)) = self.consume(
                    &LogosToken::Identifier(String::new()),
                    "Expected type name after 'struct' keyword in expression",
                ) {
                    name.to_string()
                } else {
                    return Err(());
                };

                if let None = self.consume(
                    &LogosToken::LeftBrace,
                    "Expected '{' in struct initialization",
                ) {
                    return Err(());
                }

                let mut expressions: Vec<Expression> = Vec::new();
                loop {
                    // A right brace mark the end of the expression list.
                    if self.check(&LogosToken::RightBrace) {
                        break;
                    }
                    expressions.push(self.or()?);

                    // Expect a comma

                    if !self.match_expr(&[LogosToken::Comma]) {
                        break;
                    }
                }

                if let None = self.consume(
                    &&LogosToken::RightBrace,
                    "Unclosed '}' in struct initialization",
                ) {
                    return Err(());
                }

                Ok(Expression::Literal(Literal::StructLiteral(StructLiteral {
                    literal_type: None,
                    type_name,
                    expressions,
                })))
            }
            _ => {
                if let Some(bad_tk) = token {
                    let msg = format!("Error: Unexpected {:?}", bad_tk);
                    self.put_error_at_current_token(&msg);
                } else {
                    self.put_error_at_current_token("Error: Unexpected EOF");
                }

                Err(())
            }
        }
    }
}

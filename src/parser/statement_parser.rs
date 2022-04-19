use crate::{lexer::Token, type_system::value_type::ValueType};

use super::{
    parser::Parser,
    visitors::{
        Argument, BlockStatement, Expression, FunctionStatement, IfStatement, Literal,
        ReturnStatement, Statement, VariableAssignment, VariableDeclaration,
    },
};

impl Parser {
    pub fn parse_function_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::Function]) {
            let callee = match self.consume(
                &Token::Identifier(String::new()),
                "Expected identifier after fn.",
            ) {
                Some(Token::Identifier(id)) => id.to_string(),
                _ => {
                    return Err(());
                }
            };

            if let None = self.consume(
                &Token::LeftParenthesis,
                "Expected '(' after function identifier",
            ) {
                return Err(());
            }

            let mut args: Vec<Argument> = Vec::new();
            loop {
                let id = match self.expect(&Token::Identifier(String::new())) {
                    Some(Token::Identifier(val)) => val.to_string(),
                    _ => {
                        break;
                    }
                };

                if let None = self.consume(&Token::Colon, "Expected ':' after argument identifier.")
                {
                    return Err(());
                }

                let arg_type = match self.consume(
                    &Token::Type(ValueType::Void),
                    "Expected argument type after ':'",
                ) {
                    Some(Token::Type(t)) => t,
                    _ => {
                        return Err(());
                    }
                };

                args.push((id.clone(), *arg_type));

                if !self.match_expr(&[Token::Comma]) {
                    break;
                }
            }

            if let None = self.consume(&Token::RightParenthesis, "Expected ')' after args.") {
                return Err(());
            }

            if let None = self.consume(
                &Token::Colon,
                "Expected ':' after ')', function return type must be declared.",
            ) {
                return Err(());
            }

            let return_type =
                match self.consume(&Token::Type(ValueType::Void), "Expected <type> after ':'") {
                    Some(Token::Type(t)) => t.clone(),
                    _ => return Err(()),
                };

            if let None = self.consume(&Token::LeftBracket, "Expected block after function.") {
                return Err(());
            }

            let block = self.parse_block()?;

            return Ok(Statement::Function(FunctionStatement {
                callee,
                args: if !args.is_empty() { Some(args) } else { None },
                block,
                return_type,
            }));
        }

        self.parse_if_statement()
    }

    fn parse_block(&mut self) -> Result<BlockStatement, ()> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.match_expr(&[Token::RightBracket]) {
            statements.push(self.parse_if_statement()?);
        }

        Ok(BlockStatement { statements })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::If]) {
            let condition = self.or()?;
            let then_branch = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                println!("Expected block after if condition.");
                return Err(());
            };

            if self.match_expr(&[Token::Else]) {
                let else_branch = if let Statement::Block(b) = self.parse_block_statement()? {
                    b
                } else {
                    println!("Expected block after if condition.");
                    return Err(());
                };

                return Ok(Statement::IfStatement(IfStatement {
                    condition,
                    then_branch,
                    else_branch: Some(else_branch),
                }));
            }

            return Ok(Statement::IfStatement(IfStatement {
                condition,
                then_branch,
                else_branch: None,
            }));
        }

        Ok(self.parse_block_statement()?)
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::LeftBracket]) {
            return Ok(Statement::Block(self.parse_block()?));
        }

        self.parse_return_statement()
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::Return]) {
            let expr = self.or()?;

            if let None = self.consume(&Token::Semicolon, "Expected ';' after return expression.") {
                return Err(());
            }

            return Ok(Statement::Return(ReturnStatement { expr }));
        }

        self.parse_declaration_statement()
    }

    fn parse_declaration_statement(&mut self) -> Result<Statement, ()> {
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
                }
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

    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
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

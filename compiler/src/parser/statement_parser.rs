use crate::{lexer::Token, type_system::value_type::ValueType};

use super::{
    parser::Parser,
    visitors::{
        Argument, BlockStatement, Expression, ForStatement, FunctionStatement, IfStatement,
        Literal, ReturnStatement, Statement, VariableAssignment, VariableDeclaration,
        WhileStatement,
    },
};

impl Parser {
    fn parse_function(&mut self, exported: bool) -> Result<Statement, ()> {
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

                let arg_type = self.parse_type()?;
                // let arg_type = match self.consume(
                //     &Token::Type(ValueType::Void),
                //     "Expected argument type after ':'",
                // ) {
                //     Some(Token::Type(t)) => t,
                //     _ => {
                //         return Err(());
                //     }
                // };

                args.push((id.clone(), arg_type.clone()));

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

            if !self.match_expr(&[Token::LeftBrace]) {
                if let None =
                    self.consume(&Token::Semicolon, "Expected ';' after function declaration")
                {
                    return Err(());
                }

                return Ok(Statement::Function(FunctionStatement {
                    callee,
                    args: if !args.is_empty() { Some(args) } else { None },
                    block: None,
                    return_type,
                    is_exported: exported,
                }));
            }

            let block = self.parse_block()?;

            return Ok(Statement::Function(FunctionStatement {
                callee,
                args: if !args.is_empty() { Some(args) } else { None },
                block: Some(block),
                return_type,
                is_exported: exported,
            }));
        }

        if exported {
            println!("Expected 'fn' keyword after 'export'.");
            return Err(());
        }

        self.parse_if_statement()
    }
    pub fn parse_function_statement(&mut self) -> Result<Statement, ()> {
        let exported = self.match_expr(&[Token::Export]);
        self.parse_function(exported)
    }

    fn parse_block(&mut self) -> Result<BlockStatement, ()> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.match_expr(&[Token::RightBrace]) {
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

            if let Some(_) = self.expect(&Token::Else) {
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

        Ok(self.parse_for_statement()?)
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::For]) {
            let init_expr =
                if let Statement::VariableDeclaration(dec) = self.parse_declaration_statement()? {
                    dec
                } else {
                    println!("Expected variable declaration atfer for.");
                    return Err(());
                };

            let loop_condition = self.or()?;

            if let None = self.expect(&Token::Semicolon) {
                println!("Expected ';' after loop condition.");
                return Err(());
            }

            let next_expr = self.parse_expression_statement()?;

            let block_stmt = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                println!("Expected block statement in for statement.");
                return Err(());
            };

            return Ok(Statement::ForStatement(ForStatement {
                init_expr,
                loop_condition,
                next_expr: Box::new(next_expr),
                block_stmt,
            }));
        }

        Ok(self.parse_while_statement()?)
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::While]) {
            let condition = self.or()?;
            let loop_block = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                println!("Expected block after 'while' condition.");
                return Err(());
            };

            return Ok(Statement::WhileStatement(WhileStatement {
                condition,
                loop_block,
            }));
        }

        Ok(self.parse_block_statement()?)
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::LeftBrace]) {
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

            let variable_type = self.parse_type()?;

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

            match expr {
                Expression::Literal(Literal::Identifier(id)) => {
                    return Ok(Statement::VariableAssignment(VariableAssignment {
                        identifier: Expression::Literal(Literal::Identifier(id)),
                        new_value: rhs,
                    }));
                }
                Expression::ArrayAccess(a) => {
                    return Ok(Statement::VariableAssignment(VariableAssignment {
                        identifier: Expression::ArrayAccess(a),
                        new_value: rhs,
                    }))
                }
                _ => {
                    println!("Error: left side of assignment must be an lvalue.");
                    return Err(());
                }
            };
        }

        if let None = self.consume(&Token::Semicolon, "Expected ';' after <expression>") {
            return Err(());
        }

        Ok(Statement::Expression(expr))
    }
}

use crate::lexer::Token;

use super::{
    parser::Parser,
    visitors::{
        Argument, BlockStatement, Expression, ForStatement, FunctionStatement, IfStatement,
        Literal, ReturnStatement, Statement, StructField, StructStatement, VariableAssignment,
        VariableDeclaration, WhileStatement,
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

            let return_type = self.parse_type()?;

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

        if self.match_expr(&[Token::Struct]) {
            self.parse_struct_statement(exported)
        } else {
            if exported {
                println!("Expected 'fn' keyword after 'export'.");
                return Err(());
            }

            self.parse_if_statement()
        }
    }

    pub fn parse_function_statement(&mut self) -> Result<Statement, ()> {
        let exported = self.match_expr(&[Token::Export]);
        self.parse_function(exported)
    }

    fn parse_struct_statement(&mut self, exported: bool) -> Result<Statement, ()> {
        let type_name = if let Some(Token::Identifier(id)) = self.consume(
            &Token::Identifier(String::new()),
            "Expected <type identifier> after 'struct'.",
        ) {
            id.clone()
        } else {
            return Err(());
        };

        if let None = self.consume(
            &Token::LeftBrace,
            "Expected '{' after struct type identifier.",
        ) {
            return Err(());
        }

        let mut fields: Vec<StructField> = Vec::new();
        while let Some(Token::Identifier(name)) = self.advance() {
            // Copy here because of other mutable borrows below ...
            let field_name = name.clone();

            if let None = self.consume(&Token::Colon, "Expected ':' after field identifier.") {
                return Err(());
            }

            let field_type = self.parse_type()?;

            if let None = self.consume(&Token::Semicolon, "Expected ';' after field type name.") {
                return Err(());
            }

            fields.push((field_name, field_type));
        }

        Ok(Statement::Struct(StructStatement {
            type_name,
            fields,
            exported,
        }))
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

        Ok(self.parse_loop_statement()?)
    }

    fn parse_loop_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[Token::Loop]) {
            self.consume(&Token::LeftBrace, "Expected '{' after 'loop'");
            let loop_block = self.parse_block()?;

            // A loop is just a while true {}
            return Ok(Statement::WhileStatement(WhileStatement {
                condition: Expression::Literal(Literal::Bool(true)),
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
        if self.match_expr(&[Token::Break]) {
            if let None = self.consume(&Token::Semicolon, "Expected ';' after assigment.") {
                return Err(());
            }

            return Ok(Statement::BreakStatement);
        }

        let expr = self.or()?;

        if self.match_expr(&[Token::Equal]) {
            let rhs = self.or()?;

            if let None = self.consume(&Token::Semicolon, "Expected ';' after assigment.") {
                return Err(());
            }

            return Ok(Statement::VariableAssignment(VariableAssignment {
                identifier: expr,
                new_value: rhs,
            }));
        }

        if let None = self.consume(&Token::Semicolon, "Expected ';' after <expression>") {
            return Err(());
        }

        Ok(Statement::Expression(expr))
    }
}

use crate::lexer::LogosToken;

use super::{
    literals::Bool,
    parser::Parser,
    visitors::{
        Argument, BlockStatement, BreakStatement, Expression, ForStatement, FunctionStatement,
        IfStatement, ImportStatement, Literal, ReturnStatement, Statement, StructField,
        StructStatement, VariableAssignment, VariableDeclaration, WhileStatement,
    },
};

impl Parser {
    fn parse_function(&mut self, exported: bool) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Function]) {
            let callee = match self.consume(
                &LogosToken::Identifier(String::new()),
                "Expected identifier after fn.",
            ) {
                Some(LogosToken::Identifier(id)) => id.to_string(),
                _ => {
                    return Err(());
                }
            };

            if let None = self.consume(
                &LogosToken::LeftParenthesis,
                "Expected '(' after function identifier",
            ) {
                return Err(());
            }

            let mut args: Vec<Argument> = Vec::new();
            loop {
                let id = match self.expect(&LogosToken::Identifier(String::new())) {
                    Some(LogosToken::Identifier(val)) => val.to_string(),
                    _ => {
                        break;
                    }
                };

                if let None = self.consume(
                    &LogosToken::Colon,
                    "Expected ':' after argument identifier.",
                ) {
                    return Err(());
                }

                let arg_type = self.parse_type()?;
                args.push((id.clone(), arg_type.clone()));

                if !self.match_expr(&[LogosToken::Comma]) {
                    break;
                }
            }

            if let None = self.consume(&LogosToken::RightParenthesis, "Expected ')' after args.") {
                return Err(());
            }

            if let None = self.consume(
                &LogosToken::Colon,
                "Expected ':' after ')', function return type must be declared.",
            ) {
                return Err(());
            }

            let return_type = self.parse_type()?;

            if !self.match_expr(&[LogosToken::LeftBrace]) {
                if let None = self.consume(
                    &LogosToken::Semicolon,
                    "Expected ';' after function declaration",
                ) {
                    return Err(());
                }

                let debug_tk = self.peek_token_with_info_debug();
                return Ok(Statement::Function(FunctionStatement {
                    callee,
                    args: if !args.is_empty() { Some(args) } else { None },
                    block: None,
                    return_type,
                    is_exported: exported,
                    line: debug_tk.line_number,
                    column: debug_tk.column_number,
                    filename: self.module_path.clone(),
                }));
            }

            let block = self.parse_block()?;

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::Function(FunctionStatement {
                callee,
                args: if !args.is_empty() { Some(args) } else { None },
                block: Some(block),
                return_type,
                is_exported: exported,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        if self.match_expr(&[LogosToken::Struct]) {
            self.parse_struct_statement(exported)
        } else {
            if exported {
                self.put_error_at_current_token("Expected 'fn' keyword after 'export'.");
                return Err(());
            }

            self.parse_if_statement()
        }
    }

    pub fn parse_import_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Import]) {
            let name = if let Some(LogosToken::Quote(name)) = self.consume(
                &LogosToken::Quote(String::new()),
                "Expected a string literal after 'import' keyword.",
            ) {
                name.clone()
            } else {
                return Err(());
            };

            if self
                .consume(
                    &LogosToken::Semicolon,
                    "Expected ';' after import statement.",
                )
                .is_some()
            {
                let debug_tk = self.peek_token_with_info_debug();
                Ok(Statement::Import(ImportStatement {
                    file_path: self.module_path.clone(),
                    module_path: name,
                    line: debug_tk.line_number,
                    column: debug_tk.column_number,
                    filename: self.module_path.clone(),
                }))
            } else {
                Err(())
            }
        } else {
            self.parse_function_statement()
        }
    }

    fn parse_function_statement(&mut self) -> Result<Statement, ()> {
        let exported = self.match_expr(&[LogosToken::Export]);
        self.parse_function(exported)
    }

    fn parse_struct_statement(&mut self, exported: bool) -> Result<Statement, ()> {
        let type_name = if let Some(LogosToken::Identifier(id)) = self.consume(
            &LogosToken::Identifier(String::new()),
            "Expected <type identifier> after 'struct'.",
        ) {
            id.clone()
        } else {
            return Err(());
        };

        if let None = self.consume(
            &LogosToken::LeftBrace,
            "Expected '{' after struct type identifier.",
        ) {
            return Err(());
        }

        let mut fields: Vec<StructField> = Vec::new();
        while let Some(LogosToken::Identifier(name)) = self.advance() {
            // Copy here because of other mutable borrows below ...
            let field_name = name.clone();

            if let None = self.consume(&LogosToken::Colon, "Expected ':' after field identifier.") {
                return Err(());
            }

            let field_type = self.parse_type()?;

            if let None = self.consume(
                &LogosToken::Semicolon,
                "Expected ';' after field type name.",
            ) {
                return Err(());
            }

            fields.push((field_name, field_type));
        }

        let debug_tk = self.peek_token_with_info_debug();
        Ok(Statement::Struct(StructStatement {
            type_name,
            fields,
            exported,
            line: debug_tk.line_number,
            column: debug_tk.column_number,
            filename: self.module_path.clone(),
        }))
    }

    fn parse_block(&mut self) -> Result<BlockStatement, ()> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.match_expr(&[LogosToken::RightBrace]) {
            statements.push(self.parse_if_statement()?);
        }

        let debug_tk = self.peek_token_with_info_debug();
        Ok(BlockStatement {
            statements,
            line: debug_tk.line_number,
            column: debug_tk.column_number,
            filename: self.module_path.clone(),
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::If]) {
            let condition = self.or()?;
            let then_branch = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                self.put_error_at_current_token("Expected block after if condition.");
                return Err(());
            };

            if let Some(_) = self.expect(&LogosToken::Else) {
                let else_branch = if let Statement::Block(b) = self.parse_block_statement()? {
                    b
                } else {
                    self.put_error_at_current_token("Expected block after if condition.");
                    return Err(());
                };

                let debug_tk = self.peek_token_with_info_debug();
                return Ok(Statement::IfStatement(IfStatement {
                    condition,
                    then_branch,
                    else_branch: Some(else_branch),
                    line: debug_tk.line_number,
                    column: debug_tk.column_number,
                    filename: self.module_path.clone(),
                }));
            }

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::IfStatement(IfStatement {
                condition,
                then_branch,
                else_branch: None,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        Ok(self.parse_for_statement()?)
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::For]) {
            let init_expr =
                if let Statement::VariableDeclaration(dec) = self.parse_declaration_statement()? {
                    dec
                } else {
                    self.put_error_at_current_token("Expected variable declaration atfer for.");
                    return Err(());
                };

            let loop_condition = self.or()?;

            if let None = self.expect(&LogosToken::Semicolon) {
                self.put_error_at_current_token("Expected ';' after loop condition.");
                return Err(());
            }

            let next_expr = self.parse_expression_statement()?;

            let block_stmt = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                self.put_error_at_current_token("Expected block statement in for statement.");
                return Err(());
            };

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::ForStatement(ForStatement {
                init_expr,
                loop_condition,
                next_expr: Box::new(next_expr),
                block_stmt,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        Ok(self.parse_while_statement()?)
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::While]) {
            let condition = self.or()?;
            let loop_block = if let Statement::Block(b) = self.parse_block_statement()? {
                b
            } else {
                self.put_error_at_current_token("Expected block after 'while' condition.");
                return Err(());
            };

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::WhileStatement(WhileStatement {
                condition,
                loop_block,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        Ok(self.parse_loop_statement()?)
    }

    fn parse_loop_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Loop]) {
            self.consume(&LogosToken::LeftBrace, "Expected '{' after 'loop'");
            let loop_block = self.parse_block()?;

            let debug_tk = self.peek_token_with_info_debug();

            // A loop is just a while true {}
            return Ok(Statement::WhileStatement(WhileStatement {
                condition: Expression::Literal(Literal::Bool(Bool {
                    value: true,
                    line: debug_tk.line_number,
                    column: debug_tk.column_number,
                    filename: self.module_path.clone(),
                })),
                loop_block,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        Ok(self.parse_block_statement()?)
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::LeftBrace]) {
            return Ok(Statement::Block(self.parse_block()?));
        }

        self.parse_return_statement()
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Return]) {
            let expr = self.or()?;

            if let None = self.consume(
                &LogosToken::Semicolon,
                "Expected ';' after return expression.",
            ) {
                return Err(());
            }

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::Return(ReturnStatement {
                expr,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        self.parse_declaration_statement()
    }

    fn parse_declaration_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Let]) {
            let identifier = match self.consume(
                &LogosToken::Identifier(String::new()),
                "Expected identifier after Let",
            ) {
                Some(LogosToken::Identifier(name)) => name.clone(),
                _ => {
                    return Err(());
                }
            };

            if let None = self.consume(&LogosToken::Colon, "Expected ':' after identifier.") {
                return Err(());
            }

            let variable_type = self.parse_type()?;

            if let None = self.consume(&LogosToken::Equal, "Expected '=' after typename.") {
                return Err(());
            }

            let init_expr = self.or()?;

            if let None = self.consume(&LogosToken::Semicolon, "Expected ';' after <init_expr>.") {
                return Err(());
            }

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::VariableDeclaration(VariableDeclaration {
                identifier,
                variable_type,
                init_expr,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        if self.match_expr(&[LogosToken::Break]) {
            if let None = self.consume(&LogosToken::Semicolon, "Expected ';' after assigment.") {
                return Err(());
            }

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::BreakStatement(BreakStatement {
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        let expr = self.or()?;

        if self.match_expr(&[LogosToken::Equal]) {
            let rhs = self.or()?;

            if let None = self.consume(&LogosToken::Semicolon, "Expected ';' after assigment.") {
                return Err(());
            }

            let debug_tk = self.peek_token_with_info_debug();
            return Ok(Statement::VariableAssignment(VariableAssignment {
                identifier: expr,
                new_value: rhs,
                line: debug_tk.line_number,
                column: debug_tk.column_number,
                filename: self.module_path.clone(),
            }));
        }

        if let None = self.consume(&LogosToken::Semicolon, "Expected ';' after <expression>") {
            return Err(());
        }

        Ok(Statement::Expression(expr))
    }
}

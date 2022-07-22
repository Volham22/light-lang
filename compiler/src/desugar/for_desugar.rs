use crate::parser::visitors::{
    BlockStatement, Expression, ForStatement, FunctionStatement, IfStatement,
    MutableStatementVisitor, ReturnStatement, Statement, StructStatement, VariableAssignment,
    VariableDeclaration, WhileStatement,
};

pub struct ForDesugar;

impl ForDesugar {
    pub fn visit_stmt(&mut self, stmt: &mut Statement) -> Option<Statement> {
        match stmt {
            Statement::Function(f) => self.visit_function_statement(f),
            Statement::Block(b) => self.visit_block_statement(b),
            Statement::IfStatement(i) => self.visit_if_statement(i),
            Statement::WhileStatement(w) => self.visit_while_statement(w),
            Statement::ForStatement(f) => return Some(Self::desugar_for(f)),
            _ => {}
        };

        None
    }

    fn desugar_for(for_stmt: &mut ForStatement) -> Statement {
        let mut block = BlockStatement {
            statements: Vec::new(),
        };

        let mut while_block = BlockStatement {
            statements: Vec::new(),
        };

        while_block
            .statements
            .append(&mut for_stmt.block_stmt.statements);

        while_block.statements.push((*for_stmt.next_expr).clone());
        block
            .statements
            .push(Statement::VariableDeclaration(for_stmt.init_expr.clone()));

        let desugar_while = WhileStatement {
            condition: for_stmt.loop_condition.clone(),
            loop_block: while_block,
        };

        block
            .statements
            .push(Statement::WhileStatement(desugar_while));

        Statement::Block(block)
    }
}

impl MutableStatementVisitor<()> for ForDesugar {
    fn visit_expression_statement(&mut self, _expr: &mut Expression) -> () {
        unreachable!()
    }

    fn visit_declaration_statement(&mut self, _expr: &mut VariableDeclaration) -> () {
        unreachable!()
    }

    fn visit_assignment_statement(&mut self, _expr: &mut VariableAssignment) -> () {
        unreachable!()
    }

    fn visit_function_statement(&mut self, expr: &mut FunctionStatement) -> () {
        if let Some(b) = &mut expr.block {
            self.visit_block_statement(b);
        }
    }

    fn visit_block_statement(&mut self, expr: &mut BlockStatement) -> () {
        let mut desugared: Vec<(usize, Statement)> = Vec::new();

        for (i, stmt) in expr.statements.iter_mut().enumerate() {
            match stmt {
                Statement::ForStatement(for_stmt) => {
                    desugared.push((i, Self::desugar_for(for_stmt)))
                }
                Statement::Function(f) => self.visit_function_statement(f),
                Statement::Block(b) => self.visit_block_statement(b),
                Statement::IfStatement(i) => self.visit_if_statement(i),
                Statement::WhileStatement(w) => self.visit_while_statement(w),
                _ => { /* Does nothing ... */ }
            }
        }

        // Replace desugared elements
        for (i, new_stmt) in desugared {
            expr.statements[i] = new_stmt;
        }
    }

    fn visit_return_statement(&mut self, _return_stmt: &mut ReturnStatement) -> () {
        unreachable!()
    }

    fn visit_if_statement(&mut self, if_stmt: &mut IfStatement) -> () {
        self.visit_block_statement(&mut if_stmt.then_branch);

        if let Some(else_br) = &mut if_stmt.else_branch {
            self.visit_block_statement(else_br);
        }
    }

    fn visit_while_statement(&mut self, while_stmt: &mut WhileStatement) -> () {
        self.visit_block_statement(&mut while_stmt.loop_block)
    }

    fn visit_for_statement(&mut self, _for_stmt: &mut ForStatement) -> () {
        unreachable!()
    }

    fn visit_break_statement(&mut self) {
        unreachable!()
    }

    fn visit_struct_statement(&mut self, _stct: &StructStatement) -> () {
        unreachable!()
    }
}

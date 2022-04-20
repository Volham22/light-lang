use super::visitors::{
    Binary, BinaryLogic, BlockStatement, Call, Expression, ExpressionVisitor, FunctionStatement,
    Group, IfStatement, Literal, ReturnStatement, Statement, StatementVisitor, Unary,
    VariableAssignment, VariableDeclaration, WhileStatement,
};

struct AstPrinter;

impl AstPrinter {
    fn visit_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::VariableDeclaration(decl) => self.visit_declaration_statement(decl),
            Statement::VariableAssignment(assi) => self.visit_assignment_statement(assi),
            Statement::Function(func) => self.visit_function_statement(func),
            Statement::Block(block) => self.visit_block_statement(block),
            Statement::Return(ret) => self.visit_return_statement(ret),
            Statement::IfStatement(if_stmt) => self.visit_if_statement(if_stmt),
            Statement::WhileStatement(while_stmt) => self.visit_while_statement(while_stmt),
        }
    }

    fn visit_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(val) => self.visit_literal(val),
            Expression::Binary(val) => self.visit_binary(val),
            Expression::Group(val) => self.visit_group(val),
            Expression::BinaryLogic(val) => self.visit_binary_logic(val),
            Expression::Unary(val) => self.visit_unary(val),
            Expression::Call(call) => self.visit_call(call),
        }
    }

    fn print_body(&mut self, name: &str, expressions: &[&Box<Expression>]) {
        print!("{}: [ ", name);

        for expr in expressions {
            self.visit_expr(expr);
            print!(", ");
        }

        print!("] ");
    }
}

impl ExpressionVisitor<()> for AstPrinter {
    fn visit_literal(&mut self, literal: &super::visitors::Literal) -> () {
        match literal {
            Literal::Number(val) => print!("Literal: Number {}", val),
            Literal::Real(val) => print!("Literal: Real {}", val),
            Literal::Bool(val) => print!("Literal: Bool {}", val),
            Literal::Identifier(name) => print!("Literal: Identifier {}", name),
        }
    }

    fn visit_binary(&mut self, binary: &super::visitors::Binary) -> () {
        match binary {
            Binary::Plus(left, right) => {
                self.print_body("Plus", &[left, right]);
            }
            Binary::Minus(left, right) => {
                self.print_body("Minus", &[left, right]);
            }
            Binary::Multiply(left, right) => {
                self.print_body("Multiply", &[left, right]);
            }
            Binary::Divide(left, right) => {
                self.print_body("Divide", &[left, right]);
            }
            Binary::Modulo(left, right) => {
                self.print_body("Modulo", &[left, right]);
            }
        }
    }

    fn visit_group(&mut self, group: &Group) -> () {
        print!("Group: [");
        self.visit_expr(&group.inner_expression);
        print!(" ]");
    }

    fn visit_binary_logic(&mut self, binary: &BinaryLogic) -> () {
        match binary {
            BinaryLogic::And(left, right) => {
                self.print_body("And", &[left, right]);
            }
            BinaryLogic::Or(left, right) => {
                self.print_body("Or", &[&left, &right]);
            }
            BinaryLogic::Equal(left, right) => {
                self.print_body("Equal", &[&left, &right]);
            }
            BinaryLogic::NotEqual(left, right) => {
                self.print_body("Not Equal", &[&left, &right]);
            }
            BinaryLogic::More(left, right) => {
                self.print_body("More", &[&left, &right]);
            }
            BinaryLogic::MoreEqual(left, right) => {
                self.print_body("More Equal", &[&left, &right]);
            }
            BinaryLogic::Less(left, right) => {
                self.print_body("Less", &[&left, &right]);
            }
            BinaryLogic::LessEqual(left, right) => {
                self.print_body("Less Equal", &[left, right]);
            }
        }
    }

    fn visit_unary(&mut self, unary: &super::visitors::Unary) -> () {
        match unary {
            Unary::Not(val) => {
                self.print_body("Not", &[val]);
            }
            Unary::Negate(val) => {
                self.print_body("Negate", &[val]);
            }
        }
    }

    fn visit_call(&mut self, call_expr: &Call) -> () {
        print!("call {}(", &call_expr.name);

        if let Some(args) = &call_expr.args {
            for arg in args {
                self.visit_expr(arg);
            }
        }

        print!(") ")
    }
}

impl StatementVisitor<()> for AstPrinter {
    fn visit_expression_statement(&mut self, expr: &Expression) -> () {
        print!("Statement: [");
        self.visit_expr(expr);
        println!(" ]");
    }

    fn visit_declaration_statement(&mut self, expr: &VariableDeclaration) -> () {
        print!(
            "Declaration: [identifier: {}, typename: {:?}, init_expr: ",
            expr.identifier, expr.variable_type
        );
        self.visit_expr(&expr.init_expr);
        println!("] ");
    }

    fn visit_assignment_statement(&mut self, expr: &VariableAssignment) -> () {
        print!("Assigment: [identifier: {}, new_expr: ", expr.identifier);
        self.visit_expr(&expr.new_value);
        println!("] ");
    }

    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> () {
        print!("Function {}(", expr.callee);

        if let Some(args) = &expr.args {
            for arg in args {
                print!("{:?}, ", arg);
            }
        }

        print!(")\n");
    }

    fn visit_block_statement(&mut self, expr: &BlockStatement) -> () {
        print!("Block: [");

        for stmt in &expr.statements {
            self.visit_stmt(&stmt);
        }

        print!("]")
    }

    fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) -> () {
        print!("Return: [");
        self.visit_expr(&return_stmt.expr);
        print!("]\n");
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> () {
        print!("If: ");
        self.visit_expr(&if_stmt.condition);
        print!("then: ");
        self.visit_block_statement(&if_stmt.then_branch);

        if let Some(else_b) = &if_stmt.else_branch {
            print!("else: ");
            self.visit_block_statement(else_b);
            println!(" endif");
        } else {
            println!(" endif");
        }
    }

    fn visit_while_statement(&mut self, while_stmt: &WhileStatement) -> () {
        print!(" while ");
        self.visit_expr(&while_stmt.condition);
        print!(" block: ");
        self.visit_block_statement(&while_stmt.loop_block);
        print!(" endwhile\n");
    }
}

pub fn print_ast(stmts: &Vec<Statement>) {
    let mut printer = AstPrinter {};

    for stmt in stmts {
        printer.visit_stmt(stmt);
    }
}

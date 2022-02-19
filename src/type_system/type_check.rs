use crate::{
    parser::visitors::{
        Binary, BinaryLogic, BlockStatement, Call, Expression, ExpressionVisitor,
        FunctionStatement, Group, Literal, ReturnStatement, Statement, StatementVisitor, Unary,
        VariableAssignment, VariableDeclaration,
    },
    type_system::value_type::ValueType,
};
use std::collections::HashMap;

pub struct FunctionSignature {
    name: String,
    return_type: ValueType,
    args_type: Vec<ValueType>,
}

pub struct TypeChecker {
    variables_table: Vec<HashMap<String, ValueType>>,
    function_table: HashMap<String, FunctionSignature>,
    in_function: Option<ValueType>,
}

pub type TypeCheckerReturn = Result<ValueType, String>;

impl TypeChecker {
    pub fn new() -> Self {
        let mut s = Self {
            variables_table: Vec::new(),
            function_table: HashMap::new(),
            in_function: None,
        };

        s.variables_table.push(HashMap::new()); // default global scope
        s
    }

    pub fn check_ast_type(&mut self, stmts: &Vec<Statement>) -> TypeCheckerReturn {
        for stmt in stmts {
            match stmt {
                Statement::Expression(expr) => self.visit_expression_statement(expr)?,
                Statement::VariableDeclaration(var_dec) => {
                    self.visit_declaration_statement(var_dec)?
                }
                Statement::VariableAssignment(var_ass) => {
                    self.visit_assignment_statement(var_ass)?
                }
                Statement::Function(f) => self.visit_function_statement(f)?,
                Statement::Block(b) => self.visit_block_statement(b)?,
                Statement::Return(ret) => self.visit_return_statement(ret)?,
            };
        }

        Ok(ValueType::Number)
    }

    fn check_expr(&mut self, expr: &Expression) -> TypeCheckerReturn {
        match expr {
            Expression::Literal(e) => self.visit_literal(&e),
            Expression::Binary(e) => self.visit_binary(&e),
            Expression::Group(e) => self.visit_group(&e),
            Expression::BinaryLogic(e) => self.visit_binary_logic(&e),
            Expression::Unary(e) => self.visit_unary(&e),
            Expression::Call(e) => self.visit_call(&e),
        }
    }

    fn visit_boxed_expr(&mut self, expr: &Box<Expression>) -> TypeCheckerReturn {
        match &**expr {
            Expression::Literal(e) => self.visit_literal(&e),
            Expression::Binary(e) => self.visit_binary(&e),
            Expression::Group(e) => self.visit_group(&e),
            Expression::BinaryLogic(e) => self.visit_binary_logic(&e),
            Expression::Unary(e) => self.visit_unary(&e),
            Expression::Call(e) => self.visit_call(&e),
        }
    }

    fn unpack_binary_type(
        &mut self,
        lhs: &Box<Expression>,
        rhs: &Box<Expression>,
    ) -> (TypeCheckerReturn, TypeCheckerReturn) {
        (self.visit_boxed_expr(lhs), self.visit_boxed_expr(rhs))
    }

    fn are_expressions_compatible(
        &mut self,
        l: &Box<Expression>,
        r: &Box<Expression>,
    ) -> TypeCheckerReturn {
        let (lhs_result, rhs_result) = self.unpack_binary_type(l, r);

        if let Ok(lhs_type) = lhs_result {
            if let Ok(rhs_type) = rhs_result {
                if ValueType::is_compatible(lhs_type, rhs_type) {
                    Ok(lhs_type)
                } else {
                    Err(format!(
                        "Type {} is not compatible with type {}. Consider casting.",
                        lhs_type, rhs_type
                    ))
                }
            } else {
                Err(format!("{}", rhs_result.unwrap_err()))
            }
        } else {
            Err(format!("{}", lhs_result.unwrap_err()))
        }
    }

    fn find_variable_type(&self, name: &String) -> Option<&ValueType> {
        for scope in self.variables_table.iter().rev() {
            if scope.contains_key(name) {
                return Some(scope.get(name).unwrap());
            }
        }

        None
    }

    fn add_variables_in_scope(&mut self, args: &Vec<(String, ValueType)>) {
        self.variables_table.push(HashMap::new());
        let last = self.variables_table.last_mut().unwrap();

        for (name, arg_type) in args {
            last.insert(name.to_string(), *arg_type);
        }
    }
}

impl StatementVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_expression_statement(&mut self, expr: &Expression) -> TypeCheckerReturn {
        self.check_expr(expr)
    }

    fn visit_declaration_statement(&mut self, expr: &VariableDeclaration) -> TypeCheckerReturn {
        let init_type = self.check_expr(&expr.init_expr)?;

        if !ValueType::is_compatible(init_type, expr.variable_type) {
            let message = format!(
                "variable '{}' is declared as '{}' but init expression has type '{}'",
                expr.identifier, expr.variable_type, init_type
            );

            return Err(message);
        }

        if let Some(_) = self.find_variable_type(&expr.identifier) {
            return Err(format!("Redifinition of variable '{}'.", expr.identifier));
        }

        self.variables_table
            .last_mut()
            .unwrap()
            .insert(expr.identifier.clone(), expr.variable_type);
        Ok(init_type)
    }

    fn visit_assignment_statement(&mut self, expr: &VariableAssignment) -> TypeCheckerReturn {
        if let None = self.find_variable_type(&expr.identifier) {
            return Err(format!(
                "'{}' is not declared. Declare it 'let {}: <typename> = <init_expr>;'",
                expr.identifier, expr.identifier
            ));
        }

        let expr_type = self.check_expr(&expr.new_value)?;
        let variable_type = self.find_variable_type(&expr.identifier).unwrap();

        if !ValueType::is_compatible(expr_type, *variable_type) {
            return Err(format!(
                "Cannot assign expression of type '{}' to variable '{}' of type '{}'.",
                expr_type, expr.identifier, variable_type
            ));
        }

        Ok(expr_type)
    }

    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> TypeCheckerReturn {
        if let Some(_) = self.in_function {
            return Err(format!("Nested function is not allowed."));
        }

        self.variables_table
            .first_mut()
            .unwrap()
            .insert(expr.callee.to_string(), expr.return_type);

        self.function_table.insert(
            expr.callee.to_string(),
            FunctionSignature {
                args_type: if expr.args.is_some() {
                    expr.args.as_ref().unwrap().iter().map(|e| e.1).collect()
                } else {
                    Vec::new()
                },
                name: expr.callee.to_string(),
                return_type: expr.return_type,
            },
        );

        if expr.args.is_some() {
            self.add_variables_in_scope(&expr.args.as_ref().unwrap());
        }

        self.in_function = Some(expr.return_type);
        self.visit_block_statement(&expr.block)?;
        self.in_function = None;
        self.variables_table.pop().unwrap();

        // TODO:
        // for now we only check if the function body has a return statement if
        // the return type is not 'void'. In the future it may be better to have
        // a smarter way to check that every path in the function leads to a
        // valid return statement

        if expr.return_type != ValueType::Void {
            for stmt in &expr.block.statements {
                match stmt {
                    Statement::Return(_) => {
                        return Ok(expr.return_type);
                    }
                    _ => continue,
                }
            }

            return Err(format!("Function '{}' returns no values", expr.callee));
        }

        Ok(expr.return_type)
    }

    fn visit_block_statement(&mut self, expr: &BlockStatement) -> TypeCheckerReturn {
        self.variables_table.push(HashMap::new());
        self.check_ast_type(&expr.statements)?;
        self.variables_table.pop().unwrap();

        // A block has no return type
        Ok(ValueType::Void)
    }

    fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) -> TypeCheckerReturn {
        if self.in_function.is_none() {
            return Err(format!("Return statement is valid only in a function."));
        }

        let expr_type = self.check_expr(&return_stmt.expr)?;
        let return_type = self.in_function.unwrap();

        if !ValueType::is_compatible(expr_type, return_type) {
            return Err(format!(
                "Returned '{}' is not compatible with function return type '{}'",
                expr_type, return_type
            ));
        }

        return Ok(expr_type);
    }
}

impl ExpressionVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_literal(&mut self, literal: &Literal) -> TypeCheckerReturn {
        match literal {
            Literal::Number(_) => Ok(ValueType::Number),
            Literal::Real(_) => Ok(ValueType::Real),
            Literal::Bool(_) => Ok(ValueType::Bool),
            Literal::Identifier(identifier) => {
                if let Some(var_type) = self.find_variable_type(identifier) {
                    Ok(*var_type)
                } else {
                    Err(format!(
                        "'{}' is not declared. Declare it 'let {}: <typename> = <init_expr>;'",
                        identifier, identifier
                    ))
                }
            }
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> TypeCheckerReturn {
        let is_compatible = match binary {
            Binary::Plus(l, r) => self.are_expressions_compatible(l, r),
            Binary::Minus(l, r) => self.are_expressions_compatible(l, r),
            Binary::Multiply(l, r) => self.are_expressions_compatible(l, r),
            Binary::Divide(l, r) => self.are_expressions_compatible(l, r),
            Binary::Modulo(l, r) => self.are_expressions_compatible(l, r),
        };

        if let Ok(t) = is_compatible {
            Ok(t)
        } else {
            Err(is_compatible.unwrap_err())
        }
    }

    fn visit_group(&mut self, group: &Group) -> TypeCheckerReturn {
        self.visit_boxed_expr(&group.inner_expression)
    }

    fn visit_binary_logic(&mut self, binary: &BinaryLogic) -> TypeCheckerReturn {
        let is_compatible = match binary {
            BinaryLogic::And(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::Or(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::Less(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::More(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::LessEqual(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::MoreEqual(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::Equal(l, r) => self.are_expressions_compatible(l, r),
            BinaryLogic::NotEqual(l, r) => self.are_expressions_compatible(l, r),
        };

        if let Ok(t) = is_compatible {
            Ok(ValueType::Bool)
        } else {
            Err(is_compatible.unwrap_err().to_string())
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> TypeCheckerReturn {
        match unary {
            Unary::Not(e) => self.visit_boxed_expr(e),
            Unary::Negate(e) => self.visit_boxed_expr(e),
        }
    }

    fn visit_call(&mut self, call_expr: &Call) -> TypeCheckerReturn {
        if !self.function_table.contains_key(&call_expr.name) {
            return Err(format!(
                "Function '{}' is not declared in this module.",
                &call_expr.name
            ));
        }

        if call_expr.args.is_some() {
            let expected_arg_count = self
                .function_table
                .get(&call_expr.name)
                .unwrap()
                .args_type
                .len();

            let fn_name = self
                .function_table
                .get(&call_expr.name)
                .unwrap()
                .name
                .to_string();

            let call_arg_count = call_expr.args.as_ref().unwrap().len();

            if call_arg_count != expected_arg_count {
                return Err(format!(
                    "Expected {} arguments for function '{}' call but got {} arguments.",
                    expected_arg_count, fn_name, call_arg_count
                ));
            }

            for (i, arg_expr) in call_expr.args.as_ref().unwrap().iter().enumerate() {
                let expr_type = self.check_expr(arg_expr)?;
                let fn_args = &self.function_table.get(&call_expr.name).unwrap().args_type;

                if !ValueType::is_compatible(expr_type, fn_args[i]) {
                    return Err(format!(
                        "Expression of type '{}' cannot be applied to function argument of type '{}' in the call to '{}'",
                        expr_type, fn_args[i], fn_name
                    ));
                }
            }
        }

        Ok(self
            .function_table
            .get(&call_expr.name)
            .unwrap()
            .return_type)
    }
}

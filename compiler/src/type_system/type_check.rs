use crate::{
    parser::visitors::{
        AddressOf, ArrayAccess, Binary, BinaryLogic, BlockStatement, Call, DeReference, Expression,
        ExpressionVisitor, ForStatement, FunctionStatement, Group, IfStatement, Literal,
        ReturnStatement, Statement, StatementVisitor, Unary, VariableAssignment,
        VariableDeclaration, WhileStatement,
    },
    type_system::value_type::ValueType,
};
use std::{collections::HashMap, ops::Deref};

pub struct FunctionSignature {
    name: String,
    return_type: ValueType,
    args_type: Vec<ValueType>,
}

pub struct TypeChecker {
    variables_table: Vec<HashMap<String, ValueType>>,
    function_table: HashMap<String, FunctionSignature>,
    in_function: Option<ValueType>,
    loop_count: u32,
}

pub type TypeCheckerReturn = Result<ValueType, String>;

impl TypeChecker {
    pub fn new() -> Self {
        let mut s = Self {
            variables_table: Vec::new(),
            function_table: HashMap::new(),
            in_function: None,
            loop_count: 0,
        };

        s.variables_table.push(HashMap::new()); // default global scope
        s
    }

    pub fn check_ast_type(&mut self, stmts: &Vec<Statement>) -> TypeCheckerReturn {
        for stmt in stmts {
            self.visit_statement(stmt)?;
        }

        Ok(ValueType::Number)
    }

    fn find_variable(&self, identifier: &String) -> Option<ValueType> {
        for frame in self.variables_table.iter().rev() {
            if let Some(ty) = frame.get(identifier) {
                return Some(ty.clone());
            }
        }

        None
    }

    fn visit_statement(&mut self, stmt: &Statement) -> TypeCheckerReturn {
        match stmt {
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::VariableDeclaration(var_dec) => self.visit_declaration_statement(var_dec),
            Statement::VariableAssignment(var_ass) => self.visit_assignment_statement(var_ass),
            Statement::Function(f) => self.visit_function_statement(f),
            Statement::Block(b) => self.visit_block_statement(b),
            Statement::Return(ret) => self.visit_return_statement(ret),
            Statement::IfStatement(if_stmt) => self.visit_if_statement(if_stmt),
            Statement::WhileStatement(while_stmt) => self.visit_while_statement(while_stmt),
            Statement::ForStatement(for_stmt) => self.visit_for_statement(for_stmt),
            Statement::BreakStatement => self.visit_break_statement(),
        }
    }

    fn check_expr(&mut self, expr: &Expression) -> TypeCheckerReturn {
        match expr {
            Expression::Literal(e) => self.visit_literal(&e),
            Expression::Binary(e) => self.visit_binary(&e),
            Expression::Group(e) => self.visit_group(&e),
            Expression::BinaryLogic(e) => self.visit_binary_logic(&e),
            Expression::Unary(e) => self.visit_unary(&e),
            Expression::Call(e) => self.visit_call(&e),
            Expression::ArrayAccess(a) => self.visit_array_access(&a),
            Expression::Null => self.visit_null_expression(),
            Expression::AddressOf(address_of) => self.visit_address_of_expression(address_of),
            Expression::DeReference(deref) => self.visit_dereference_expression(deref),
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
            Expression::ArrayAccess(a) => self.visit_array_access(&a),
            Expression::Null => self.visit_null_expression(),
            Expression::AddressOf(address_of) => self.visit_address_of_expression(address_of),
            Expression::DeReference(deref) => self.visit_dereference_expression(deref),
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

        if let Ok(lhs_type) = &lhs_result {
            if let Ok(rhs_type) = &rhs_result {
                if ValueType::is_compatible(lhs_type, rhs_type) {
                    Ok(lhs_type.clone())
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
            last.insert(name.to_string(), arg_type.clone());
        }
    }

    fn check_array_element_assignment(
        &mut self,
        access: &ArrayAccess,
        rhs: &Expression,
    ) -> TypeCheckerReturn {
        if let Some(array_dec) = self.find_variable(&access.identifier) {
            let rhs_ty = self.check_expr(rhs)?;

            if let ValueType::Array(arr) = &array_dec {
                if ValueType::is_compatible(arr.array_type.deref(), &rhs_ty) {
                    Ok(rhs_ty)
                } else {
                    Err(format!(
                        "Can't assign expression of type '{}' to array element of type '{}'",
                        rhs_ty, array_dec
                    ))
                }
            } else {
                Err(format!(
                    "Can't assign expression of type '{}' to array element of type '{}'",
                    rhs_ty, array_dec
                ))
            }
        } else {
            Err(format!("Array '{}' is not declared.", access.identifier))
        }
    }

    fn check_simple_assignment(
        &mut self,
        identifier: &String,
        rhs: &Expression,
    ) -> TypeCheckerReturn {
        let expr_type = self.check_expr(rhs)?;
        let variable_type = if let Some(v) = self.find_variable_type(identifier) {
            v
        } else {
            return Err(format!("Undeclared variable '{}'", identifier));
        };

        if !ValueType::is_compatible(&expr_type, variable_type) {
            return Err(format!(
                "Cannot assign expression of type '{}' to variable '{}' of type '{}'.",
                expr_type, identifier, variable_type
            ));
        }

        Ok(expr_type)
    }
}

impl StatementVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_expression_statement(&mut self, expr: &Expression) -> TypeCheckerReturn {
        self.check_expr(expr)
    }

    fn visit_declaration_statement(&mut self, expr: &VariableDeclaration) -> TypeCheckerReturn {
        let init_type = self.check_expr(&expr.init_expr)?;

        if !ValueType::is_compatible_for_init(&expr.variable_type, &init_type) {
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
            .insert(expr.identifier.clone(), expr.variable_type.clone());
        Ok(init_type)
    }

    fn visit_assignment_statement(&mut self, expr: &VariableAssignment) -> TypeCheckerReturn {
        match &expr.identifier {
            Expression::Literal(Literal::Identifier(identifier)) => {
                self.check_simple_assignment(&identifier, &expr.new_value)
            }
            Expression::ArrayAccess(access) => {
                self.check_array_element_assignment(&access, &expr.new_value)
            }
            Expression::DeReference(deref) => {
                let deref_ty = self.visit_dereference_expression(deref)?;
                let init_ty = self.check_expr(&expr.new_value)?;

                if !ValueType::is_compatible(&deref_ty, &init_ty) {
                    return Err(format!(
                        "Cannot assign type '{}' with a dereferenced pointer of type '{}'",
                        init_ty, deref_ty
                    ));
                }

                Ok(deref_ty)
            }
            _ => Err(format!("Assignment left hand side is not an lvalue.")),
        }
    }

    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> TypeCheckerReturn {
        if let Some(_) = self.in_function {
            return Err(format!("Nested function is not allowed."));
        }

        self.variables_table
            .first_mut()
            .unwrap()
            .insert(expr.callee.to_string(), expr.return_type.clone());

        self.function_table.insert(
            expr.callee.to_string(),
            FunctionSignature {
                args_type: if expr.args.is_some() {
                    expr.args
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|e| e.1.clone())
                        .collect()
                } else {
                    Vec::new()
                },
                name: expr.callee.to_string(),
                return_type: expr.return_type.clone(),
            },
        );

        if expr.args.is_some() {
            self.add_variables_in_scope(&expr.args.as_ref().unwrap());
        }

        self.in_function = Some(expr.return_type.clone());

        if let Some(b) = &expr.block {
            self.visit_block_statement(&b)?;

            if expr.args.is_some() {
                self.variables_table.pop(); // remove arguments scope if any
            }

            // TODO:
            // for now we only check if the function body has a return statement if
            // the return type is not 'void'. In the future it may be better to have
            // a smarter way to check that every path in the function leads to a
            // valid return statement
            if expr.return_type != ValueType::Void {
                for stmt in &b.statements {
                    match stmt {
                        Statement::Return(_) => {
                            self.in_function = None;
                            return Ok(expr.return_type.clone());
                        }
                        _ => continue,
                    }
                }

                self.in_function = None;
                return Err(format!("Function '{}' returns no values", expr.callee));
            }
        } else {
            self.in_function = None;

            // A function must have a block definition if exported
            if expr.is_exported {
                return Err(format!("Error: exported function must have a body."));
            }
        }

        self.in_function = None;
        Ok(expr.return_type.clone())
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
        let return_type = self.in_function.as_ref().unwrap();

        if !ValueType::is_compatible(&expr_type, &return_type) {
            return Err(format!(
                "Returned '{}' is not compatible with function return type '{}'",
                expr_type, return_type
            ));
        }

        return Ok(expr_type);
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> TypeCheckerReturn {
        let condition_type = self.check_expr(&if_stmt.condition)?;

        if condition_type != ValueType::Bool {
            return Err(format!(
                "If condition has type '{}' but the type bool is needed.",
                condition_type
            ));
        }

        self.visit_block_statement(&if_stmt.then_branch)?;

        if if_stmt.else_branch.is_some() {
            self.visit_block_statement(&if_stmt.else_branch.as_ref().unwrap())?;
        }

        // An if statement itself has void type
        Ok(ValueType::Void)
    }

    fn visit_while_statement(&mut self, while_stmt: &WhileStatement) -> TypeCheckerReturn {
        let condition_type = self.visit_expression_statement(&while_stmt.condition)?;
        self.loop_count += 1;

        if condition_type != ValueType::Bool {
            return Err(format!(
                "While condition has type '{}' but the type bool is needed.",
                condition_type
            ));
        }

        self.visit_block_statement(&while_stmt.loop_block)?;
        self.loop_count -= 1;

        // An while statement has void type
        Ok(ValueType::Void)
    }

    fn visit_for_statement(&mut self, for_stmt: &ForStatement) -> TypeCheckerReturn {
        // A for loop declare a variable (ie. i) and this variable needs its
        // own scope to avoid false positive redifinitions
        self.variables_table.push(HashMap::new());
        self.loop_count += 1;

        let init_type = self.visit_declaration_statement(&for_stmt.init_expr)?;
        let loop_type = self.visit_expression_statement(&for_stmt.loop_condition)?;
        self.visit_statement(&for_stmt.next_expr)?;
        self.visit_block_statement(&for_stmt.block_stmt)?;

        // Pop the for's variable scope here it's not needed and can lead to
        // false positive variable redefinitions errors
        self.variables_table.pop();
        self.loop_count -= 1;

        if init_type != ValueType::Number && init_type != ValueType::Real {
            return Err(format!(
                "For init declaration has type '{}' but type 'number' or 'real' is required.",
                init_type
            ));
        }

        if loop_type != ValueType::Bool {
            return Err(format!(
                "For loop expression has type '{}' but type 'bool' is required.",
                loop_type
            ));
        }

        Ok(ValueType::Void)
    }

    fn visit_break_statement(&mut self) -> TypeCheckerReturn {
        if self.loop_count == 0 {
            return Err(format!("Break statement outside a loop."));
        }

        Ok(ValueType::Void)
    }
}

impl ExpressionVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_literal(&mut self, literal: &Literal) -> TypeCheckerReturn {
        match literal {
            Literal::Number(_) => Ok(ValueType::Number),
            Literal::Real(_) => Ok(ValueType::Real),
            Literal::Bool(_) => Ok(ValueType::Bool),
            Literal::StringLiteral(_) => Ok(ValueType::String),
            Literal::Identifier(identifier) => {
                if let Some(var_type) = self.find_variable_type(identifier) {
                    Ok(var_type.clone())
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

        if let Ok(_) = is_compatible {
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

                if !ValueType::is_compatible(&expr_type, &fn_args[i]) {
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
            .return_type
            .clone())
    }

    fn visit_array_access(&mut self, call_expr: &ArrayAccess) -> TypeCheckerReturn {
        if let Some(ValueType::Array(arr)) = self.find_variable(&call_expr.identifier) {
            Ok(*arr.array_type)
        } else {
            Err(format!("Undeclared array '{}'", call_expr.identifier))
        }
    }

    fn visit_null_expression(&mut self) -> TypeCheckerReturn {
        Ok(ValueType::Null)
    }

    fn visit_address_of_expression(&mut self, address_of: &AddressOf) -> TypeCheckerReturn {
        if let Some(ty) = self.find_variable_type(&address_of.identifier) {
            Ok(ValueType::Pointer(Box::new(ty.clone())))
        } else {
            Err(format!("Undeclared variable '{}'", &address_of.identifier))
        }
    }

    fn visit_dereference_expression(&mut self, dereference: &DeReference) -> TypeCheckerReturn {
        if let Some(ValueType::Pointer(ptr_ty)) = self.find_variable_type(&dereference.identifier) {
            Ok(*ptr_ty.clone())
        } else {
            Err(format!(
                "'{}' is either not a pointer or declared in this scope.",
                &dereference.identifier
            ))
        }
    }
}

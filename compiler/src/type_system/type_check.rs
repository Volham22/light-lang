use crate::{
    parser::visitors::{
        ArrayAccess, Expression, ExpressionVisitor, Statement, StatementVisitor, StructLiteral,
        StructStatement,
    },
    type_system::value_type::ValueType,
};
use std::{collections::HashMap, ops::Deref};

use super::types_table::TypeTable;

pub struct FunctionSignature {
    pub name: String,
    pub return_type: ValueType,
    pub args_type: Vec<ValueType>,
}

pub struct TypeChecker {
    pub(super) structs_table: HashMap<String, StructStatement>,
    pub(super) variables_table: Vec<HashMap<String, ValueType>>,
    pub(super) function_table: HashMap<String, FunctionSignature>,
    pub(super) in_function: Option<ValueType>,
    pub(super) loop_count: u32,
    pub(super) type_table: TypeTable,
}

pub type TypeCheckerReturn = Result<ValueType, String>;

impl TypeChecker {
    pub fn new() -> Self {
        let mut s = Self {
            structs_table: HashMap::new(),
            variables_table: Vec::new(),
            function_table: HashMap::new(),
            in_function: None,
            loop_count: 0,
            type_table: TypeTable::new(),
        };

        s.variables_table.push(HashMap::new()); // default global scope
        s
    }

    pub fn get_type_table(&self) -> TypeTable {
        self.type_table.clone()
    }

    pub fn check_ast_type(&mut self, stmts: &Vec<Statement>) -> TypeCheckerReturn {
        for stmt in stmts {
            self.visit_statement(stmt)?;
        }

        Ok(ValueType::Number)
    }

    pub fn find_variable(&self, identifier: &String) -> Option<ValueType> {
        for frame in self.variables_table.iter().rev() {
            if let Some(ty) = frame.get(identifier) {
                return Some(ty.clone());
            }
        }

        None
    }

    pub fn visit_statement(&mut self, stmt: &Statement) -> TypeCheckerReturn {
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
            Statement::Struct(struct_stmt) => self.visit_struct_statement(struct_stmt),
        }
    }

    pub fn check_expr(&mut self, expr: &Expression) -> TypeCheckerReturn {
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
            Expression::MemberAccess(member_access) => self.visit_member_access(member_access),
        }
    }

    pub fn visit_boxed_expr(&mut self, expr: &Box<Expression>) -> TypeCheckerReturn {
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
            Expression::MemberAccess(member_access) => self.visit_member_access(member_access),
        }
    }

    pub fn unpack_binary_type(
        &mut self,
        lhs: &Box<Expression>,
        rhs: &Box<Expression>,
    ) -> (TypeCheckerReturn, TypeCheckerReturn) {
        (self.visit_boxed_expr(lhs), self.visit_boxed_expr(rhs))
    }

    pub fn are_expressions_compatible(
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

    pub fn find_variable_type(&self, name: &String) -> Option<&ValueType> {
        for scope in self.variables_table.iter().rev() {
            if scope.contains_key(name) {
                return Some(scope.get(name).unwrap());
            }
        }

        None
    }

    pub fn add_variables_in_scope(&mut self, args: &Vec<(String, ValueType)>) {
        self.variables_table.push(HashMap::new());
        let last = self.variables_table.last_mut().unwrap();

        for (name, arg_type) in args {
            self.type_table.add_variable(name, arg_type);
            last.insert(name.to_string(), arg_type.clone());
        }
    }

    pub fn check_array_element_assignment(
        &mut self,
        access: &ArrayAccess,
        rhs: &Expression,
    ) -> TypeCheckerReturn {
        if let Some(array_dec) = self.find_variable(&access.identifier) {
            let rhs_ty = self.check_expr(rhs)?;

            match &array_dec {
                ValueType::Array(arr) => {
                    if ValueType::is_compatible(arr.array_type.deref(), &rhs_ty) {
                        Ok(rhs_ty)
                    } else {
                        Err(format!(
                            "Can't assign expression of type '{}' to array element of type '{}'",
                            rhs_ty, array_dec
                        ))
                    }
                }
                ValueType::Pointer(ptr_ty) => {
                    if ValueType::is_compatible(ptr_ty, &rhs_ty) {
                        Ok(rhs_ty)
                    } else {
                        Err(format!(
                            "Can't assign expression of type '{}' to array element of type '{}'",
                            rhs_ty, array_dec
                        ))
                    }
                }
                _ => Err(format!("Can't assign on a non-array type '{}'", &array_dec)),
            }
        } else {
            Err(format!("Array '{}' is not declared.", access.identifier))
        }
    }

    pub fn check_simple_assignment(
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

    pub fn check_valid_struct_literal(
        &mut self,
        struct_literal: &StructLiteral,
    ) -> TypeCheckerReturn {
        let struct_dec = if let Some(dec) = self.structs_table.get(&struct_literal.type_name) {
            dec.clone()
        } else {
            return Err(format!("Undeclared struct '{}'", struct_literal.type_name));
        };

        if struct_dec.fields.len() != struct_literal.expressions.len() {
            return Err(format!("Incorrect number of expressions to init struct '{}', got {} expressions but {} are required.", struct_literal.type_name,
                               struct_literal.expressions.len(), struct_dec.fields.len()));
        }

        for (i, expr) in struct_literal.expressions.iter().enumerate() {
            let expr_type = self.check_expr(expr)?;

            if !ValueType::is_compatible_for_init(&struct_dec.fields[i].1, &expr_type) {
                return Err(format!(
                    "In struct '{}' literal, can't init type {} with type {} at position {}",
                    struct_literal.type_name,
                    struct_dec.fields[i].1,
                    expr_type,
                    i + 1
                ));
            }
        }

        Ok(ValueType::Struct(struct_dec.type_name))
    }
}

use std::{collections::HashMap, str::FromStr};

use crate::parser::visitors::{
    Binary, BinaryLogic, Expression, ExpressionVisitor, Group, Literal, Statement,
    StatementVisitor, Unary, VariableAssignment, VariableDeclaration,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ValueType {
    Number,
    Real,
    Bool,
    String,
}

impl ValueType {
    fn is_compatible(ltype: ValueType, rtype: ValueType) -> bool {
        rtype == ltype
    }
}

impl FromStr for ValueType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "number" => Ok(ValueType::Number),
            "real" => Ok(ValueType::Real),
            "bool" => Ok(ValueType::Bool),
            "string" => Ok(ValueType::String),
            _ => Err("Unkown type"),
        }
    }
}

pub struct TypeChecker {
    variables_table: HashMap<String, ValueType>,
}

pub type TypeCheckerReturn = Result<ValueType, String>;

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            variables_table: HashMap::new(),
        }
    }

    pub fn check_ast_type(&mut self, stmts: &Vec<Statement>) -> TypeCheckerReturn {
        for stmt in stmts {
            match stmt {
                Statement::Expression(expr) => self.visit_expression_statement(expr)?,
                Statement::VariableDeclaration(var_dec) => self.visit_declaration_statement(var_dec)?,
                Statement::VariableAssignment(var_ass) => self.visit_assignment_statement(var_ass)?,
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
        }
    }

    fn visit_boxed_expr(&mut self, expr: &Box<Expression>) -> TypeCheckerReturn {
        match &**expr {
            Expression::Literal(e) => self.visit_literal(&e),
            Expression::Binary(e) => self.visit_binary(&e),
            Expression::Group(e) => self.visit_group(&e),
            Expression::BinaryLogic(e) => self.visit_binary_logic(&e),
            Expression::Unary(e) => self.visit_unary(&e),
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
                        "Type {:?} is not compatible with type {:?}. Consider casting.",
                        lhs_type, rhs_type
                    ))
                }
            } else {
                Err(rhs_result.unwrap_err())
            }
        } else {
            Err(lhs_result.unwrap_err())
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
                "variable '{}' is declared as '{:?}' but init expression has type '{:?}'",
                expr.identifier, expr.variable_type, init_type
            );

            return Err(message);
        }

        if self.variables_table.contains_key(&expr.identifier) {
            return Err(format!("Redifinition of variable '{}'.", expr.identifier));
        }

        self.variables_table
            .insert(expr.identifier.clone(), expr.variable_type);
        Ok(init_type)
    }

    fn visit_assignment_statement(&mut self, expr: &VariableAssignment) -> TypeCheckerReturn {
        if !self.variables_table.contains_key(&expr.identifier) {
            return Err(format!(
                "'{}' is not declared. Declare it 'let {}: <typename> = <init_expr>;'",
                expr.identifier, expr.identifier
            ));
        }

        let expr_type = self.check_expr(&expr.new_value)?;
        let variable_type = self.variables_table.get(&expr.identifier).unwrap();

        if !ValueType::is_compatible(expr_type, *variable_type) {
            return Err(format!(
                "Cannot assign expression of type '{:?}' to variable '{}' of type '{:?}'.",
                expr_type, expr.identifier, variable_type
            ));
        }

        Ok(expr_type)
    }
}

impl ExpressionVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_literal(&mut self, literal: &Literal) -> TypeCheckerReturn {
        match literal {
            Literal::Number(_) => Ok(ValueType::Number),
            Literal::Real(_) => Ok(ValueType::Real),
            Literal::Bool(_) => Ok(ValueType::Bool),
            Literal::Identifier(identifier) => {
                if self.variables_table.contains_key(identifier) {
                    Ok(*self.variables_table.get(identifier).unwrap())
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
            Ok(t)
        } else {
            Err(is_compatible.unwrap_err())
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> TypeCheckerReturn {
        match unary {
            Unary::Not(e) => self.visit_boxed_expr(e),
            Unary::Negate(e) => self.visit_boxed_expr(e),
        }
    }
}

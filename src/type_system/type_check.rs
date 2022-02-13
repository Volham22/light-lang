use std::str::FromStr;

use crate::parser::visitors::{
    Binary, BinaryLogic, Expression, ExpressionVisitor, Group, Literal, Unary,
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

struct TypeChecker;

pub type TypeCheckerReturn = Result<ValueType, String>;

impl TypeChecker {
    pub fn check_expr(&mut self, expr: &Expression) -> TypeCheckerReturn {
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

impl ExpressionVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_literal(&mut self, literal: &Literal) -> TypeCheckerReturn {
        match literal {
            Literal::Number(_) => Ok(ValueType::Number),
            Literal::Real(_) => Ok(ValueType::Real),
            Literal::Bool(_) => Ok(ValueType::Bool),
            Literal::Identifier(_) => todo!("implement variables"),
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

pub fn check_expression_type(expression: &Expression) -> TypeCheckerReturn {
    let mut checker = TypeChecker {};

    Ok(checker.check_expr(expression)?)
}

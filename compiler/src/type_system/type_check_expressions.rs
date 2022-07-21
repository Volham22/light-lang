use crate::parser::visitors::{
    AddressOf, ArrayAccess, Binary, BinaryLogic, Call, DeReference, ExpressionVisitor, Group,
    Literal, MemberAccess, StructLiteral, Unary,
};

use super::{
    type_check::{TypeChecker, TypeCheckerReturn},
    value_type::ValueType,
};

impl ExpressionVisitor<Result<ValueType, String>> for TypeChecker {
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
            Literal::StructLiteral(s) => self.check_valid_struct_literal(s),
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
        if let Some(id) = self.find_variable(&call_expr.identifier) {
            match id {
                ValueType::Array(arr_ty) => Ok(*arr_ty.array_type),
                ValueType::Pointer(ptr_ty) => Ok(*ptr_ty),
                _ => Err(format!(
                    "'{}' is not a subscriptable type.",
                    &call_expr.identifier
                )),
            }
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

    fn visit_struct_literal(
        &mut self,
        struct_literal: &StructLiteral,
    ) -> Result<ValueType, String> {
        todo!()
    }

    fn visit_member_access(&mut self, member_access: &MemberAccess) -> Result<ValueType, String> {
        let declaration_type = match self.find_variable(&member_access.object) {
            Some(var) => {
                if let ValueType::Struct(struct_name) = var {
                    if let Some(ty) = self.structs_table.get(&struct_name) {
                        ty
                    } else {
                        unreachable!("Struct declared with an undeclared type!??")
                    }
                } else {
                    return Err(format!(
                        "Variable '{}' is not a struct, the dot operator can't be applied on it.",
                        &member_access.object
                    ));
                }
            }
            None => {
                return Err(format!("Undeclared variable '{}'", member_access.object));
            }
        };

        if let Some(field) = declaration_type
            .fields
            .iter()
            .find(|f| f.0 == member_access.member)
        {
            Ok(field.1.clone())
        } else {
            Err(format!(
                "Type '{}' (accessed from variable '{}') has no field '{}'",
                declaration_type.type_name, member_access.object, member_access.member
            ))
        }
    }
}

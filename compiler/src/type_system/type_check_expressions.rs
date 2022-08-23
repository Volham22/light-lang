use crate::parser::visitors::{
    AddressOf, ArrayAccess, Binary, BinaryLogic, Call, DeReference, Group, Literal, MemberAccess,
    MutableExpressionVisitor, StructLiteral, Unary,
};

use super::{
    type_check::{TypeChecker, TypeCheckerReturn},
    typed::Typed,
    value_type::ValueType,
};

impl MutableExpressionVisitor<Result<ValueType, String>> for TypeChecker {
    fn visit_literal(&mut self, literal: &mut Literal) -> TypeCheckerReturn {
        match literal {
            Literal::Number(_) => Ok(ValueType::Number),
            Literal::Real(_) => Ok(ValueType::Real),
            Literal::Bool(_) => Ok(ValueType::Bool),
            Literal::Char(_) => Ok(ValueType::Char),
            Literal::StringLiteral(_) => Ok(ValueType::String),
            Literal::Identifier(identifier) => {
                if let Some(var_type) = self.find_variable_type(&identifier.name) {
                    identifier.is_lvalue = self.is_lvalue;
                    identifier.set_type(var_type.clone());
                    Ok(var_type.clone())
                } else {
                    Err(format!(
                        "'{}' is not declared. Declare it 'let {}: <typename> = <init_expr>;'",
                        identifier, identifier
                    ))
                }
            }
            Literal::StructLiteral(s) => self.visit_struct_literal(s),
        }
    }

    fn visit_binary(&mut self, binary: &mut Binary) -> TypeCheckerReturn {
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

    fn visit_group(&mut self, group: &mut Group) -> TypeCheckerReturn {
        self.visit_boxed_expr(&mut group.inner_expression)
    }

    fn visit_binary_logic(&mut self, binary: &mut BinaryLogic) -> TypeCheckerReturn {
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

    fn visit_unary(&mut self, unary: &mut Unary) -> TypeCheckerReturn {
        match unary {
            Unary::Not(e) => self.visit_boxed_expr(e),
            Unary::Negate(e) => self.visit_boxed_expr(e),
        }
    }

    fn visit_call(&mut self, call_expr: &mut Call) -> TypeCheckerReturn {
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

            for (i, arg_expr) in call_expr
                .args
                .as_deref_mut()
                .unwrap()
                .iter_mut()
                .enumerate()
            {
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

        let call_type = self
            .function_table
            .get(&call_expr.name)
            .unwrap()
            .return_type
            .clone();

        call_expr.set_type(call_type.clone());

        Ok(call_type)
    }

    fn visit_array_access(&mut self, array_access: &mut ArrayAccess) -> TypeCheckerReturn {
        let id_ty = self.check_expr(&mut array_access.identifier)?;

        match id_ty {
            ValueType::Array(arr_ty) => {
                array_access.set_type(arr_ty.array_type.as_ref().clone());
                Ok(*arr_ty.array_type)
            }
            ValueType::String => {
                array_access.set_type(ValueType::Char);
                Ok(ValueType::Char)
            }
            ValueType::Pointer(ptr_ty) => {
                array_access.set_type(ptr_ty.as_ref().clone());
                Ok(*ptr_ty)
            }
            _ => Err(format!("'{}' is not a subscriptable type.", id_ty)),
        }
    }

    fn visit_null_expression(&mut self) -> TypeCheckerReturn {
        Ok(ValueType::Null)
    }

    fn visit_address_of_expression(&mut self, address_of: &mut AddressOf) -> TypeCheckerReturn {
        let identifier_ty = self.check_expr(&mut address_of.identifier)?;

        match identifier_ty {
            ValueType::Array(a) => Ok(ValueType::Pointer(Box::new(ValueType::Array(a)))),
            ValueType::Bool => Ok(ValueType::Pointer(Box::new(ValueType::Bool))),
            ValueType::Number => Ok(ValueType::Pointer(Box::new(ValueType::Number))),
            ValueType::Real => Ok(ValueType::Pointer(Box::new(ValueType::Real))),
            ValueType::String => Ok(ValueType::Pointer(Box::new(ValueType::String))),
            ValueType::Char => Ok(ValueType::Pointer(Box::new(ValueType::Char))),
            ValueType::Function => Err(format!("Function pointers are not supported yet.")),
            ValueType::Pointer(ptr) => Ok(ValueType::Pointer(Box::new(ValueType::Pointer(ptr)))),
            ValueType::Struct(strct) => Ok(ValueType::Pointer(Box::new(ValueType::Struct(strct)))),
            ValueType::Void => Err(format!("Addrof cannot be applied to void types.")),
            ValueType::Null => Err(format!("Addrof 'null' is forbidden.")),
        }
    }

    fn visit_dereference_expression(&mut self, dereference: &mut DeReference) -> TypeCheckerReturn {
        let deref_ty = self.check_expr(&mut dereference.identifier)?;

        if let ValueType::Pointer(ptr) = deref_ty {
            dereference.set_type(*ptr.clone());
            dereference.is_lvalue = self.is_lvalue;
            Ok(*ptr.clone())
        } else {
            Err(format!(
                "'{}' Cannot be dereferenced as it's not a pointer type.",
                deref_ty
            ))
        }
    }

    fn visit_struct_literal(
        &mut self,
        struct_literal: &mut StructLiteral,
    ) -> Result<ValueType, String> {
        let ty = self.check_valid_struct_literal(struct_literal)?;
        struct_literal.set_type(ty.clone());

        Ok(ty)
    }

    fn visit_member_access(
        &mut self,
        member_access: &mut MemberAccess,
    ) -> Result<ValueType, String> {
        let declaration_type = match &self.visit_boxed_expr(&mut member_access.object)? {
            ValueType::Struct(s) => {
                member_access.set_type(ValueType::Struct(s.to_string()));
                self.structs_table.get(s).unwrap()
            }
            ValueType::Pointer(ty) => match ty.as_ref() {
                ValueType::Struct(s) => {
                    member_access.set_type(ValueType::Struct(s.to_string()));
                    self.structs_table.get(s).unwrap()
                }
                _ => {
                    return Err(format!("Member access on a non-struct type"));
                }
            },
            _ => {
                return Err(format!("Member access on a non-struct type"));
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
                "Type '{}' has no field '{}'",
                declaration_type.type_name, member_access.member
            ))
        }
    }
}

use std::collections::HashMap;

use crate::parser::visitors::{
    BlockStatement, BreakStatement, Expression, ForStatement, FunctionStatement, IfStatement,
    ImportStatement, MutableExpressionVisitor, MutableStatementVisitor, ReturnStatement, Statement,
    StructStatement, VariableAssignment, VariableDeclaration, WhileStatement,
};

use super::{
    type_check::{FunctionSignature, TypeChecker, TypeCheckerReturn},
    value_type::ValueType,
};

impl MutableStatementVisitor<TypeCheckerReturn> for TypeChecker {
    fn visit_expression_statement(&mut self, expr: &mut Expression) -> TypeCheckerReturn {
        self.check_expr(expr)
    }

    fn visit_declaration_statement(&mut self, expr: &mut VariableDeclaration) -> TypeCheckerReturn {
        let init_type = self.check_expr(&mut expr.init_expr)?;

        if !ValueType::is_compatible_for_init(&expr.variable_type, &init_type) {
            return Err(Self::build_error_message(
                format!(
                    "variable '{}' is declared as '{}' but init expression has type '{}'",
                    expr.identifier, expr.variable_type, init_type
                )
                .as_str(),
                expr,
            ));
        }

        if let Some(_) = self.find_variable_type(&expr.identifier) {
            return Err(Self::build_error_message(
                format!("Redifinition of variable '{}'.", expr.identifier).as_str(),
                expr,
            ));
        }

        self.variables_table
            .last_mut()
            .unwrap()
            .insert(expr.identifier.clone(), expr.variable_type.clone());

        self.type_table
            .add_variable(&expr.identifier, &expr.variable_type);

        Ok(init_type)
    }

    fn visit_assignment_statement(&mut self, expr: &mut VariableAssignment) -> TypeCheckerReturn {
        match &mut expr.identifier {
            Expression::ArrayAccess(access) => {
                access.is_lvalue = true;
                self.check_array_element_assignment(access, &mut expr.new_value)
            }
            Expression::DeReference(deref) => {
                deref.is_lvalue = true;
                let deref_ty = self.visit_dereference_expression(deref)?;
                let init_ty = self.check_expr(&mut expr.new_value)?;

                if !ValueType::is_compatible(&deref_ty, &init_ty) {
                    return Err(Self::build_error_message(
                        format!(
                            "Cannot assign type '{}' with a dereferenced pointer of type '{}'",
                            init_ty, deref_ty
                        )
                        .as_str(),
                        expr,
                    ));
                }

                Ok(deref_ty)
            }
            Expression::MemberAccess(member_access) => {
                self.is_lvalue = true;
                let member_ty = self.visit_member_access(member_access)?;
                let init_ty = self.check_expr(&mut expr.new_value)?;
                self.is_lvalue = false;

                if !ValueType::is_compatible(&member_ty, &init_ty) {
                    return Err(Self::build_error_message(
                        format!(
                            "Cannot assign on member '{}' of type '{}' with type '{}'",
                            member_access.member, member_ty, init_ty
                        )
                        .as_str(),
                        expr,
                    ));
                }

                Ok(member_ty)
            }
            _ => {
                self.is_lvalue = true;
                let result =
                    self.check_simple_assignment(&mut expr.identifier, &mut expr.new_value);
                self.is_lvalue = false;

                result
            }
        }
    }

    fn visit_function_statement(&mut self, expr: &mut FunctionStatement) -> TypeCheckerReturn {
        if let Some(_) = self.in_function {
            return Err(Self::build_error_message(
                format!("Nested function is not allowed.").as_str(),
                expr,
            ));
        }

        self.variables_table
            .first_mut()
            .unwrap()
            .insert(expr.callee.to_string(), expr.return_type.clone());

        self.type_table.add_function_type(expr);

        // TODO: Function pointer support
        self.type_table
            .add_variable(&expr.callee, &ValueType::Function);

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

        if let Some(b) = &mut expr.block {
            self.visit_block_statement(b)?;

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
                return Err(Self::build_error_message(
                    format!("Function '{}' returns no values", expr.callee).as_str(),
                    expr,
                ));
            }
        } else {
            self.in_function = None;
        }

        self.in_function = None;
        Ok(expr.return_type.clone())
    }

    fn visit_block_statement(&mut self, expr: &mut BlockStatement) -> TypeCheckerReturn {
        self.variables_table.push(HashMap::new());
        self.check_ast_type(&mut expr.statements)?;
        self.variables_table.pop().unwrap();

        // A block has no return type
        Ok(ValueType::Void)
    }

    fn visit_return_statement(&mut self, return_stmt: &mut ReturnStatement) -> TypeCheckerReturn {
        if self.in_function.is_none() {
            return Err(Self::build_error_message(
                format!("Return statement is valid only in a function.").as_str(),
                return_stmt,
            ));
        }

        let expr_type = self.check_expr(&mut return_stmt.expr)?;
        let return_type = self.in_function.as_ref().unwrap();

        if !ValueType::is_compatible(&expr_type, &return_type) {
            return Err(Self::build_error_message(
                format!(
                    "Returned '{}' is not compatible with function return type '{}'",
                    expr_type, return_type
                )
                .as_str(),
                return_stmt,
            ));
        }

        return Ok(expr_type);
    }

    fn visit_if_statement(&mut self, if_stmt: &mut IfStatement) -> TypeCheckerReturn {
        let condition_type = self.check_expr(&mut if_stmt.condition)?;

        if condition_type != ValueType::Bool {
            return Err(Self::build_error_message(
                format!(
                    "If condition has type '{}' but the type bool is needed.",
                    condition_type
                )
                .as_str(),
                if_stmt,
            ));
        }

        self.visit_block_statement(&mut if_stmt.then_branch)?;

        if if_stmt.else_branch.is_some() {
            self.visit_block_statement(&mut if_stmt.else_branch.as_mut().unwrap())?;
        }

        // An if statement itself has void type
        Ok(ValueType::Void)
    }

    fn visit_while_statement(&mut self, while_stmt: &mut WhileStatement) -> TypeCheckerReturn {
        let condition_type = self.visit_expression_statement(&mut while_stmt.condition)?;
        self.loop_count += 1;

        if condition_type != ValueType::Bool {
            return Err(Self::build_error_message(
                format!(
                    "While condition has type '{}' but the type bool is needed.",
                    condition_type
                )
                .as_str(),
                while_stmt,
            ));
        }

        self.visit_block_statement(&mut while_stmt.loop_block)?;
        self.loop_count -= 1;

        // An while statement has void type
        Ok(ValueType::Void)
    }

    fn visit_for_statement(&mut self, for_stmt: &mut ForStatement) -> TypeCheckerReturn {
        // A for loop declare a variable (ie. i) and this variable needs its
        // own scope to avoid false positive redifinitions
        self.variables_table.push(HashMap::new());
        self.loop_count += 1;

        let init_type = self.visit_declaration_statement(&mut for_stmt.init_expr)?;
        let loop_type = self.visit_expression_statement(&mut for_stmt.loop_condition)?;
        self.visit_statement(&mut for_stmt.next_expr)?;
        self.visit_block_statement(&mut for_stmt.block_stmt)?;

        // Pop the for's variable scope here it's not needed and can lead to
        // false positive variable redefinitions errors
        self.variables_table.pop();
        self.loop_count -= 1;

        if init_type != ValueType::Number && init_type != ValueType::Real {
            return Err(Self::build_error_message(
                format!(
                    "For init declaration has type '{}' but type 'number' or 'real' is required.",
                    init_type
                )
                .as_str(),
                for_stmt,
            ));
        }

        if loop_type != ValueType::Bool {
            return Err(Self::build_error_message(
                format!(
                    "For loop expression has type '{}' but type 'bool' is required.",
                    loop_type
                )
                .as_str(),
                for_stmt,
            ));
        }

        Ok(ValueType::Void)
    }

    fn visit_break_statement(&mut self, break_stmt: &mut BreakStatement) -> TypeCheckerReturn {
        if self.loop_count == 0 {
            return Err(Self::build_error_message(
                format!("Break statement outside a loop.").as_str(),
                break_stmt,
            ));
        }

        Ok(ValueType::Void)
    }

    fn visit_struct_statement(&mut self, stct: &StructStatement) -> TypeCheckerReturn {
        if self.structs_table.contains_key(&stct.type_name) {
            return Err(Self::build_error_message(
                format!("Redefinition of struct '{}'", &stct.type_name).as_str(),
                stct,
            ));
        }

        self.structs_table
            .insert(stct.type_name.clone(), stct.clone());
        self.type_table.add_struct_type(stct);

        Ok(ValueType::Struct(stct.type_name.clone()))
    }

    fn visit_import_statement(&mut self, _import_stmt: &mut ImportStatement) -> TypeCheckerReturn {
        unreachable!("Import statememts presents in type check stage!");
    }
}

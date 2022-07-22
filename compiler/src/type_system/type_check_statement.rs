use std::collections::HashMap;

use crate::parser::visitors::{
    BlockStatement, Expression, ExpressionVisitor, ForStatement, FunctionStatement, IfStatement,
    Literal, ReturnStatement, Statement, StatementVisitor, StructStatement, VariableAssignment,
    VariableDeclaration, WhileStatement,
};

use super::{
    type_check::{FunctionSignature, TypeChecker, TypeCheckerReturn},
    value_type::ValueType,
};

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

        self.type_table
            .add_variable(&expr.identifier, &expr.variable_type);

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
            Expression::MemberAccess(member_access) => {
                let member_ty = self.visit_member_access(member_access)?;
                let init_ty = self.check_expr(&expr.new_value)?;

                if !ValueType::is_compatible(&member_ty, &init_ty) {
                    return Err(format!(
                        "Cannot assign on member '{}' of type '{}' with type '{}'",
                        member_access.member, member_ty, init_ty
                    ));
                }

                Ok(member_ty)
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

    fn visit_struct_statement(&mut self, stct: &StructStatement) -> TypeCheckerReturn {
        if self.structs_table.contains_key(&stct.type_name) {
            return Err(format!("Redefinition of struct '{}'", &stct.type_name));
        }

        self.structs_table
            .insert(stct.type_name.clone(), stct.clone());
        self.type_table.add_struct_type(stct);

        Ok(ValueType::Struct(stct.type_name.clone()))
    }
}

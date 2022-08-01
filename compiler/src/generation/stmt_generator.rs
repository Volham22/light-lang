use std::{borrow::Borrow, ops::Deref};

use inkwell::{
    module::Linkage,
    types::{ArrayType, BasicMetadataTypeEnum, BasicType, BasicTypeEnum},
    values::{AnyValue, AnyValueEnum, BasicValue, BasicValueEnum, PointerValue},
    AddressSpace,
};

use crate::{
    parser::visitors::{
        BlockStatement, Expression, ForStatement, FunctionStatement, IfStatement, ImportStatement,
        Literal, ReturnStatement, StatementVisitor, StructStatement, VariableAssignment,
        VariableDeclaration, WhileStatement,
    },
    type_system::value_type::{StaticArray, ValueType},
};

use super::ir_generator::IRGenerator;

impl<'a> IRGenerator<'a> {
    fn generate_block_instructions(&mut self, block: &BlockStatement) {
        for stmt in &block.statements {
            self.visit_statement(&stmt);
        }
    }

    pub fn get_llvm_array_type(&self, array_type: &StaticArray) -> ArrayType<'a> {
        match array_type.array_type.deref() {
            ValueType::Array(a) => self.get_llvm_array_type(a),
            ValueType::Number => self.context.i64_type().array_type(array_type.size as u32),
            ValueType::Real => self.context.f64_type().array_type(array_type.size as u32),
            ValueType::Bool => self.context.bool_type().array_type(array_type.size as u32),
            ValueType::String => self
                .context
                .i8_type()
                .ptr_type(inkwell::AddressSpace::Generic)
                .array_type(array_type.size as u32),
            ValueType::Function => todo!(),
            ValueType::Void => unreachable!(),
            ValueType::Pointer(_) => todo!(),
            ValueType::Null => unreachable!(),
            ValueType::Struct(_) => todo!(),
        }
    }

    pub fn get_concrete_array_type(&self, array_type: &StaticArray) -> BasicTypeEnum<'a> {
        match array_type.array_type.deref() {
            ValueType::Array(a) => self.get_concrete_array_type(a),
            ValueType::Number => self
                .context
                .i64_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Real => self
                .context
                .f64_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Bool => self
                .context
                .bool_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::String => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Pointer(_) => todo!(),
            ValueType::Function => todo!(),
            ValueType::Void => unreachable!("array type can't be void!"),
            ValueType::Null => unreachable!("Array type of null!"),
            ValueType::Struct(_) => todo!(),
        }
    }

    fn allocate_array(&self, array_type: &StaticArray, array_name: &str) -> PointerValue<'a> {
        let size_value = self
            .context
            .i64_type()
            .const_int(array_type.size as u64, false);

        match array_type.array_type.deref() {
            ValueType::Number => self.builder.build_array_alloca(
                self.context.i64_type().array_type(array_type.size as u32),
                size_value,
                array_name,
            ),
            ValueType::Real => {
                self.builder
                    .build_array_alloca(self.context.f64_type(), size_value, array_name)
            }
            ValueType::Bool => self
                .builder
                .build_array_alloca(self.context.bool_type(), size_value, array_name)
                .into(),
            _ => todo!(),
        }
    }

    fn init_array<T: BasicValue<'a> + Copy>(
        &self,
        array_ptr: PointerValue<'a>,
        array_type: &StaticArray,
        init_value: T,
    ) {
        let concrete_type = self.get_concrete_array_type(array_type).into_pointer_type();

        let array_ptr_cast = self
            .builder
            .build_bitcast(array_ptr, concrete_type, "array_cast")
            .into_pointer_value();

        for i in 0..array_type.size {
            let offset_ptr = unsafe {
                self.builder.build_gep(
                    array_ptr_cast,
                    &[self.context.i64_type().const_int(i as u64, false)],
                    "array_init",
                )
            };

            self.builder.build_store(offset_ptr, init_value);
        }
    }

    fn declare_and_init_array(
        &mut self,
        array_type: &StaticArray,
        init_value: &AnyValueEnum<'a>,
        array_name: &String,
    ) {
        let array_ptr = self.allocate_array(array_type, array_name.as_str());

        self.variables.insert(array_name.to_string(), array_ptr);

        match array_type.array_type.deref() {
            ValueType::Number => {
                let init_value = self.get_int_value(*init_value);
                self.init_array(array_ptr, array_type, init_value)
            }
            ValueType::Real => {
                let init_value = self.get_float_value(*init_value);
                self.init_array(array_ptr, array_type, init_value)
            }
            ValueType::Bool => {
                let init_value = self.get_int_value(*init_value);
                self.init_array(array_ptr, array_type, init_value)
            }
            _ => todo!(),
        }
    }

    /// If the lhs is an array cast it to the correct array
    /// element pointer and build the store
    fn build_assignment<T: BasicValue<'a> + Copy>(
        &mut self,
        val_ptr: &PointerValue<'a>,
        new_value: T,
        lhs_expr: &Expression,
    ) -> Option<AnyValueEnum<'a>> {
        if val_ptr.get_type().get_element_type().is_array_type() {
            let array_type = self
                .builder
                .build_bitcast(
                    *val_ptr,
                    val_ptr
                        .get_type()
                        .get_element_type()
                        .into_array_type()
                        .get_element_type()
                        .ptr_type(AddressSpace::Generic),
                    "store_array_element_cast",
                )
                .into_pointer_value();

            let array_access = if let Expression::ArrayAccess(a) = lhs_expr {
                a
            } else {
                panic!();
            };
            let index_value = self.visit_expr(&array_access.index).into_int_value();

            let offset_ptr = unsafe {
                self.builder
                    .build_gep(array_type, &[index_value], "array_assign_gep")
            };

            Some(self.builder.build_store(offset_ptr, new_value).into())
        } else {
            Some(self.builder.build_store(*val_ptr, new_value).into())
        }
    }
}

impl<'a> StatementVisitor<Option<AnyValueEnum<'a>>> for IRGenerator<'a> {
    fn visit_expression_statement(&mut self, expr: &Expression) -> Option<AnyValueEnum<'a>> {
        if let Expression::Literal(Literal::Identifier(name)) = expr {
            match self.variables.get(&name.name) {
                Some(val) => self.builder.build_load(*val, name.name.as_str()),
                None => panic!("{} doest not exists in IR generation abort.", name),
            };
        }

        None
    }

    fn visit_declaration_statement(
        &mut self,
        var_dec: &VariableDeclaration,
    ) -> Option<AnyValueEnum<'a>> {
        if let ValueType::Array(a) = &var_dec.variable_type {
            let init_value = self.visit_borrowed_expr(&var_dec.init_expr);
            self.declare_and_init_array(&a, &init_value, &var_dec.identifier);
            return None;
        }

        let init_expr = self.visit_borrowed_expr(&var_dec.init_expr);
        let val_ptr =
            self.create_entry_block_alloca(var_dec.identifier.as_str(), &var_dec.variable_type);
        self.variables
            .insert(var_dec.identifier.to_string(), val_ptr);

        match init_expr {
            AnyValueEnum::IntValue(v) => {
                self.builder.build_store(val_ptr, v);
            }
            AnyValueEnum::FloatValue(v) => {
                self.builder.build_store(val_ptr, v);
            }
            AnyValueEnum::PointerValue(v) => {
                self.builder.build_store(val_ptr, v);
            }
            AnyValueEnum::StructValue(v) => {
                self.builder.build_store(val_ptr, v);
            }
            _ => panic!(),
        };

        None
    }

    fn visit_assignment_statement(
        &mut self,
        var_ass: &VariableAssignment,
    ) -> Option<AnyValueEnum<'a>> {
        let new_expr = self.visit_borrowed_expr(&var_ass.new_value);
        let val_ptr = match &var_ass.identifier {
            Expression::Literal(Literal::Identifier(id)) => {
                self.variables.get(&id.name).unwrap().clone()
            }
            Expression::ArrayAccess(access) => {
                let ptr_val = self.visit_expr(&access.identifier);

                // Array is an pointer
                if ptr_val.get_type().is_pointer_type() {
                    let array_ptr = self
                        .builder
                        .build_load(ptr_val.into_pointer_value(), "load_array_ptr");
                    let index_value = self.visit_expr(&access.index);

                    unsafe {
                        self.builder.build_gep(
                            array_ptr.into_pointer_value(),
                            &[index_value.into_int_value()],
                            "array_ptr_gep",
                        )
                    }
                } else {
                    ptr_val.into_pointer_value()
                }
            }
            Expression::MemberAccess(member_access) => {
                let value = self.visit_expr(&member_access.object);
                self.get_struct_member_pointer_value(member_access, value.into_pointer_value())
            }
            _ => self
                .visit_borrowed_expr(&var_ass.identifier)
                .into_pointer_value(),
        };

        match new_expr {
            AnyValueEnum::IntValue(v) => self.build_assignment(&val_ptr, v, &var_ass.identifier),
            AnyValueEnum::FloatValue(v) => self.build_assignment(&val_ptr, v, &var_ass.identifier),
            AnyValueEnum::PointerValue(v) => {
                self.build_assignment(&val_ptr, v, &var_ass.identifier)
            }
            _ => panic!(),
        };

        None
    }

    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> Option<AnyValueEnum<'a>> {
        let args_type = if expr.args.is_some() {
            Some(
                expr.args
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|t| -> BasicMetadataTypeEnum {
                        match &t.1 {
                            ValueType::Number => self.context.i64_type().into(),
                            ValueType::Real => self.context.f64_type().into(),
                            ValueType::Bool => self.context.bool_type().into(),
                            ValueType::String => self
                                .context
                                .i8_type()
                                .ptr_type(inkwell::AddressSpace::Generic)
                                .into(),
                            ValueType::Function => todo!(),
                            ValueType::Void => todo!(),
                            ValueType::Array(arr) => self.get_concrete_array_type(arr).into(),
                            ValueType::Pointer(ptr) => {
                                self.get_ptr_type(&self.get_llvm_type(ptr)).into()
                            }
                            ValueType::Null => unreachable!("Parameter of type null!"),
                            ValueType::Struct(strct) => self
                                .struct_types
                                .get(strct)
                                .unwrap()
                                .as_basic_type_enum()
                                .into(),
                        }
                    })
                    .collect::<Vec<BasicMetadataTypeEnum>>(),
            )
        } else {
            None
        };

        let fn_type = match &expr.return_type {
            ValueType::Number => self.context.i64_type().fn_type(
                if args_type.is_some() {
                    args_type.as_ref().unwrap().as_slice()
                } else {
                    &[]
                },
                false,
            ),
            ValueType::Real => self.context.f64_type().fn_type(
                if args_type.is_some() {
                    args_type.as_ref().unwrap().as_slice()
                } else {
                    &[]
                },
                false,
            ),
            ValueType::Bool => self.context.bool_type().fn_type(
                if args_type.is_some() {
                    args_type.as_ref().unwrap().as_slice()
                } else {
                    &[]
                },
                false,
            ),
            ValueType::Void => self.context.void_type().fn_type(
                if args_type.is_some() {
                    args_type.as_ref().unwrap().as_slice()
                } else {
                    &[]
                },
                false,
            ),
            ValueType::Function => todo!(),
            ValueType::String => todo!(),
            ValueType::Array(_) => todo!(),
            ValueType::Pointer(ptr) => self
                .get_ptr_type(&self.get_llvm_type(ptr.borrow()))
                .fn_type(
                    if args_type.is_some() {
                        args_type.as_ref().unwrap().as_slice()
                    } else {
                        &[]
                    },
                    false,
                ),
            ValueType::Null => unreachable!("null return type!"),
            ValueType::Struct(_) => todo!(),
        };

        let fn_val = self.module.add_function(
            expr.callee.as_str(),
            fn_type,
            // Main is implicitly exported
            if expr.is_exported || expr.callee == "main" {
                Some(Linkage::External)
            } else {
                Some(Linkage::Internal)
            },
        );

        self.current_fn = Some(fn_val);

        // If it's a function definition
        if let Some(b) = &expr.block {
            let entry = self.context.append_basic_block(fn_val, "entry");
            self.builder.position_at_end(entry);

            for (i, arg) in fn_val.get_param_iter().enumerate() {
                match arg {
                    BasicValueEnum::IntValue(v) => {
                        let (arg_name, arg_type) = expr.args.as_ref().unwrap().get(i).unwrap();
                        v.set_name(arg_name.as_str());
                        let alloca = self.create_entry_block_alloca(arg_name, arg_type);
                        self.builder.build_store(alloca, v);
                        self.variables.insert(arg_name.to_string(), alloca);
                    }
                    BasicValueEnum::FloatValue(v) => {
                        let (arg_name, arg_type) = expr.args.as_ref().unwrap().get(i).unwrap();
                        v.set_name(arg_name.as_str());
                        let alloca = self.create_entry_block_alloca(arg_name, arg_type);
                        self.builder.build_store(alloca, v);
                        self.variables.insert(arg_name.to_string(), alloca);
                    }
                    BasicValueEnum::ArrayValue(v) => {
                        let (arg_name, arg_type) = expr.args.as_ref().unwrap().get(i).unwrap();
                        v.set_name(&arg_name);
                        let alloca = self.create_entry_block_alloca(arg_name, arg_type);
                        self.builder.build_store(alloca, v);
                        self.variables.insert(arg_name.to_string(), alloca);
                    }
                    BasicValueEnum::PointerValue(v) => {
                        let (arg_name, arg_type) = expr.args.as_ref().unwrap().get(i).unwrap();
                        v.set_name(&arg_name);
                        let alloca = self.create_entry_block_alloca(arg_name, arg_type);
                        self.builder.build_store(alloca, v);
                        self.variables.insert(arg_name.to_string(), alloca);
                    }
                    BasicValueEnum::StructValue(v) => {
                        let (arg_name, arg_type) = expr.args.as_ref().unwrap().get(i).unwrap();
                        v.set_name(&arg_name);
                        let alloca = self.create_entry_block_alloca(arg_name, arg_type);
                        self.builder.build_store(alloca, v);
                        self.variables.insert(arg_name.to_string(), alloca);
                    }
                    _ => panic!(),
                }
            }
            self.generate_block_instructions(&b);
        } else {
            // else just declare the function, it has no block
            self.current_fn = None;
            return Some(fn_val.as_any_value_enum());
        }

        self.current_fn = None;

        if expr.return_type == ValueType::Void {
            self.builder.build_return(None);
        }

        fn_val.verify(true);

        Some(AnyValueEnum::FunctionValue(fn_val))
    }

    fn visit_struct_statement(&mut self, stct: &StructStatement) -> Option<AnyValueEnum<'a>> {
        let fields_type: Vec<BasicTypeEnum<'a>> = stct
            .fields
            .iter()
            .map(|f| self.get_llvm_basic_type(&f.1))
            .collect();

        let llvm_struct_ty = self
            .context
            .struct_type(fields_type.as_slice(), /* packed: */ false);

        self.struct_types
            .insert(stct.type_name.to_string(), llvm_struct_ty);

        None
    }

    fn visit_block_statement(&mut self, expr: &BlockStatement) -> Option<AnyValueEnum<'a>> {
        // let current_fn = self.current_fn.unwrap();
        // let current_fn_bb = current_fn.get_last_basic_block().unwrap();

        // let anonymous_block = self.context.append_basic_block(current_fn, "anon_block");
        // self.builder.position_at_end(anonymous_block);

        for stmt in &expr.statements {
            self.visit_statement(&stmt);
        }

        // self.builder.build_unconditional_branch(current_fn_bb);

        None
    }

    fn visit_return_statement(
        &mut self,
        return_stmt: &ReturnStatement,
    ) -> Option<AnyValueEnum<'a>> {
        let value = self.visit_borrowed_expr(&return_stmt.expr);

        match value {
            AnyValueEnum::IntValue(v) => self.builder.build_return(Some(&v)),
            AnyValueEnum::FloatValue(v) => self.builder.build_return(Some(&v)),
            _ => panic!(),
        };

        Some(value)
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> Option<AnyValueEnum<'a>> {
        let parent = if let Some(current) = self.current_fn {
            current
        } else {
            panic!("If must be in a function !")
        };

        let false_const = self.context.bool_type().const_zero();
        let condition = match self.visit_borrowed_expr(&if_stmt.condition) {
            AnyValueEnum::IntValue(v) => v,
            _ => panic!(),
        };

        let cond_instr = self.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            false_const,
            "if_condition",
        );

        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let merge_bb = self.context.append_basic_block(parent, "merge");

        self.builder
            .build_conditional_branch(cond_instr, then_bb, else_bb);

        self.builder.position_at_end(then_bb);
        self.visit_block_statement(&if_stmt.then_branch);

        if !self.block_has_branch() {
            self.builder.build_unconditional_branch(merge_bb);
        }

        self.builder.position_at_end(else_bb);

        if if_stmt.else_branch.is_some() {
            self.visit_block_statement(&if_stmt.else_branch.as_ref().unwrap());
        }

        if !self.block_has_branch() {
            self.builder.build_unconditional_branch(merge_bb);
        }

        self.builder.position_at_end(merge_bb);

        None
    }

    fn visit_while_statement(&mut self, while_stmt: &WhileStatement) -> Option<AnyValueEnum<'a>> {
        let parent = if let Some(current) = self.current_fn {
            current
        } else {
            panic!("If must be in a function !")
        };

        let false_const = self.context.bool_type().const_zero();

        let test_bb = self.context.append_basic_block(parent, "while_test");
        let body_bb = self.context.append_basic_block(parent, "while_body");
        let end_loop_bb = self.context.append_basic_block(parent, "while_end");

        self.builder.build_unconditional_branch(test_bb);
        self.builder.position_at_end(test_bb);

        let condition = match self.visit_borrowed_expr(&while_stmt.condition) {
            AnyValueEnum::IntValue(v) => v,
            _ => panic!(),
        };

        let cond_instr = self.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            false_const,
            "while_condition",
        );

        self.builder
            .build_conditional_branch(cond_instr, body_bb, end_loop_bb);

        self.loop_bb_stack.push(end_loop_bb);
        self.builder.position_at_end(body_bb);
        self.visit_block_statement(&while_stmt.loop_block);

        if !self.block_has_branch() {
            self.builder.build_unconditional_branch(test_bb);
        }

        self.loop_bb_stack.pop();

        self.builder.position_at_end(end_loop_bb);

        None
    }

    fn visit_for_statement(&mut self, _for_stmt: &ForStatement) -> Option<AnyValueEnum<'a>> {
        unreachable!()
    }

    fn visit_break_statement(&mut self) -> Option<AnyValueEnum<'a>> {
        self.builder
            .build_unconditional_branch(*self.loop_bb_stack.last().unwrap());
        self.has_branched = true;

        None
    }

    fn visit_import_statement(
        &mut self,
        _import_stmt: &ImportStatement,
    ) -> Option<AnyValueEnum<'a>> {
        unreachable!()
    }
}

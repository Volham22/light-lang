use inkwell::types::{AnyType, BasicType};
use inkwell::values::{AggregateValue, AnyValue, AnyValueEnum, BasicValue, BasicValueEnum};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};

use crate::generation::ir_generator::IRGenerator;
use crate::parser::visitors::{
    AddressOf, ArrayAccess, Binary, BinaryLogic, Call, DeReference, Expression, ExpressionVisitor,
    Group, Literal, MemberAccess, StructLiteral, Unary,
};

impl<'a> ExpressionVisitor<AnyValueEnum<'a>> for IRGenerator<'a> {
    fn visit_literal(&mut self, literal: &Literal) -> AnyValueEnum<'a> {
        match literal {
            Literal::Number(val) => self
                .context
                .i64_type()
                .const_int(*val as u64, true)
                .as_any_value_enum(),
            Literal::Real(val) => self
                .context
                .f64_type()
                .const_float(*val)
                .as_any_value_enum(),
            Literal::Bool(val) => self
                .context
                .i8_type()
                .const_int(if *val { 1 } else { 0 }, false)
                .as_any_value_enum(),
            Literal::Identifier(name) => {
                let val_ptr = self.variables.get(&name.name).unwrap();

                if val_ptr.get_type().get_element_type().is_array_type() {
                    let ty = val_ptr
                        .get_type()
                        .get_element_type()
                        .into_array_type()
                        .get_element_type()
                        .ptr_type(inkwell::AddressSpace::Generic);

                    self.builder
                        .build_bitcast(*val_ptr, ty, "array_to_array_ptr")
                        .as_any_value_enum()
                } else {
                    self.builder
                        .build_load(*val_ptr, name.name.as_str())
                        .as_any_value_enum()
                }
            }
            Literal::StringLiteral(s) => self
                .builder
                .build_global_string_ptr(s.as_str(), "string_literal")
                .as_any_value_enum(),
            Literal::StructLiteral(s) => self.visit_struct_literal(s),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> AnyValueEnum<'a> {
        match binary {
            Binary::Plus(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(left) => self
                        .builder
                        .build_int_add(left, self.get_int_value(right), "tmpiadd")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(left) => self
                        .builder
                        .build_float_add(left, self.get_float_value(right), "tmpfadd")
                        .as_any_value_enum(),
                    _ => panic!("Adding other thing than numeric values! Type checker failed?"),
                }
            }
            Binary::Minus(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(left) => self
                        .builder
                        .build_int_sub(left, self.get_int_value(right), "tmpisub")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(left) => self
                        .builder
                        .build_float_sub(left, self.get_float_value(right), "tmpfsub")
                        .as_any_value_enum(),
                    _ => panic!("Adding other thing than numeric values! Type checker failed?"),
                }
            }
            Binary::Multiply(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(left) => self
                        .builder
                        .build_int_mul(left, self.get_int_value(right), "tmpimul")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(left) => self
                        .builder
                        .build_float_mul(left, self.get_float_value(right), "tmpfmul")
                        .as_any_value_enum(),
                    _ => panic!("Sub other thing than numeric values! Type checker failed?"),
                }
            }
            Binary::Divide(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(left) => self
                        .builder
                        .build_int_signed_div(left, self.get_int_value(right), "tmpidiv")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(left) => self
                        .builder
                        .build_float_div(left, self.get_float_value(right), "tmpfdiv")
                        .as_any_value_enum(),
                    _ => panic!("Dividing other thing than numeric values! Type checker failed?"),
                }
            }
            Binary::Modulo(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(left) => self
                        .builder
                        .build_int_signed_rem(left, self.get_int_value(right), "tmpimul")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(left) => self
                        .builder
                        .build_float_rem(left, self.get_float_value(right), "tmpfmul")
                        .as_any_value_enum(),
                    _ => panic!("Rem other thing than numeric values! Type checker failed?"),
                }
            }
        }
    }

    fn visit_group(&mut self, group: &Group) -> AnyValueEnum<'a> {
        self.visit_expr(&group.inner_expression)
    }

    fn visit_binary_logic(&mut self, binary: &BinaryLogic) -> AnyValueEnum<'a> {
        match binary {
            BinaryLogic::And(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_and(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpand",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::Or(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_or(self.get_int_value(left), self.get_int_value(right), "tmpor")
                    .as_any_value_enum()
            }
            BinaryLogic::Equal(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            val,
                            self.get_int_value(right),
                            "tmpieq",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::OEQ,
                            val,
                            self.get_float_value(right),
                            "tmpfeq",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
            BinaryLogic::NotEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::NE,
                            val,
                            self.get_int_value(right),
                            "tmpineq",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::ONE,
                            val,
                            self.get_float_value(right),
                            "tmpfneq",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
            BinaryLogic::More(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::SGT,
                            val,
                            self.get_int_value(right),
                            "tmpimore",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::OGT,
                            val,
                            self.get_float_value(right),
                            "tmpfmore",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
            BinaryLogic::Less(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::SLT,
                            val,
                            self.get_int_value(right),
                            "tmpiless",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::OLT,
                            val,
                            self.get_float_value(right),
                            "tmpfless",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
            BinaryLogic::MoreEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::SGE,
                            val,
                            self.get_int_value(right),
                            "tmpimoreequal",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::OGE,
                            val,
                            self.get_float_value(right),
                            "tmpfmoreequal",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
            BinaryLogic::LessEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                match left {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_compare(
                            IntPredicate::SLE,
                            val,
                            self.get_int_value(right),
                            "tmpilessequal",
                        )
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_compare(
                            FloatPredicate::OLE,
                            val,
                            self.get_float_value(right),
                            "tmpflessequal",
                        )
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply equal on non Number values! Type checker failed?"),
                }
            }
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> AnyValueEnum<'a> {
        match unary {
            Unary::Not(expr) => {
                let expr = self.visit_expr(expr);

                match expr {
                    AnyValueEnum::IntValue(val) => {
                        self.builder.build_not(val, "tmpinot").as_any_value_enum()
                    }
                    _ => panic!("Cannot apply not on non Number values! Type checker failed?"),
                }
            }
            Unary::Negate(expr) => {
                let expr = self.visit_expr(expr);

                match expr {
                    AnyValueEnum::IntValue(val) => self
                        .builder
                        .build_int_neg(val, "tmpineg")
                        .as_any_value_enum(),
                    AnyValueEnum::FloatValue(val) => self
                        .builder
                        .build_float_neg(val, "tmpfneg")
                        .as_any_value_enum(),
                    _ => panic!("Cannot apply negate on non numeric values! Type checker failed?"),
                }
            }
        }
    }

    fn visit_call(&mut self, call_expr: &Call) -> AnyValueEnum<'a> {
        if let None = self.module.get_function(&call_expr.name) {
            panic!("Call to a function that is not declared.");
        }

        let fn_call = self.module.get_function(&call_expr.name).unwrap();
        let mut args_values = Vec::with_capacity(if call_expr.args.is_some() {
            call_expr.args.as_ref().unwrap().len()
        } else {
            0
        });

        if call_expr.args.is_some() {
            for arg in call_expr.args.as_ref().unwrap() {
                args_values.push(match self.visit_borrowed_expr(arg) {
                    AnyValueEnum::IntValue(v) => v.into(),
                    AnyValueEnum::FloatValue(v) => v.into(),
                    AnyValueEnum::PointerValue(v) => v.into(),
                    AnyValueEnum::ArrayValue(v) => self
                        .builder
                        .build_bitcast(
                            v,
                            self.get_ptr_type(&v.get_type().get_element_type().as_any_type_enum()),
                            "arg_array_ptr_cast",
                        )
                        .into(),
                    AnyValueEnum::StructValue(v) => v.into(),
                    _ => panic!(),
                });
            }
        }

        let value = self
            .builder
            .build_call(fn_call, args_values.as_slice(), "tmp_call")
            .try_as_basic_value();

        match value.left() {
            Some(v) => v.into(),
            None => match value.right() {
                Some(v) => v.into(),
                None => panic!("wrong call!"),
            },
        }
    }

    fn visit_array_access(&mut self, call_expr: &ArrayAccess) -> AnyValueEnum<'a> {
        let expr = self.visit_expr(&call_expr.index);
        let ptr = self.visit_expr(&call_expr.identifier);
        let value = self.get_int_value(expr);

        // If the array is passed as a function argument it is accessed as array ptr
        if ptr.get_type().is_array_type() {
            let ty = ptr
                .get_type()
                .into_array_type()
                .get_element_type()
                .ptr_type(inkwell::AddressSpace::Generic);

            let array_ptr =
                self.builder
                    .build_pointer_cast(ptr.into_pointer_value(), ty, "array_cast");

            let offset_ptr = unsafe {
                self.builder
                    .build_gep(array_ptr, &[value], "array_access_gep")
            };

            self.builder.build_load(offset_ptr, "load_array").into()
        } else {
            let loaded_value = self
                .builder
                .build_load(ptr.into_pointer_value(), "pre_array_ptr_load");

            let offset_ptr = unsafe {
                self.builder.build_gep(
                    loaded_value.into_pointer_value(),
                    &[value],
                    "array_access_gep",
                )
            };

            self.builder.build_load(offset_ptr, "load_array_ptr").into()
        }
    }

    fn visit_null_expression(&mut self) -> AnyValueEnum<'a> {
        self.context
            .i64_type()
            .ptr_type(inkwell::AddressSpace::Generic)
            .const_null()
            .as_any_value_enum()
    }

    fn visit_address_of_expression(&mut self, address_of: &AddressOf) -> AnyValueEnum<'a> {
        // self.variables
        //     .get(&address_of.identifier)
        //     .unwrap()
        //     .as_any_value_enum()
        match &*address_of.identifier {
            Expression::Literal(l) => match l {
                Literal::StringLiteral(sl) => self
                    .builder
                    .build_global_string_ptr(sl.as_str(), "addrof_string_literal")
                    .as_any_value_enum(),
                Literal::Identifier(id) => {
                    self.variables.get(&id.name).unwrap().as_any_value_enum()
                }
                _ => unreachable!(),
            },
            Expression::Binary(b) => self.visit_binary(&b).as_any_value_enum(),
            Expression::Group(g) => self.visit_group(&g).as_any_value_enum(),
            Expression::BinaryLogic(bl) => self.visit_binary_logic(&bl),
            Expression::Unary(u) => self.visit_unary(&u).as_any_value_enum(),
            Expression::Call(c) => self.visit_call(&c).as_any_value_enum(),
            Expression::ArrayAccess(_aa) => unreachable!(),
            Expression::AddressOf(ao) => self.visit_address_of_expression(&ao),
            Expression::DeReference(dr) => self.visit_expr(&dr.identifier),
            Expression::MemberAccess(ma) => {
                let struct_value = self.visit_expr(&ma.object);

                let struct_type = self
                    .type_table
                    .find_struct_type(ma.ty.as_ref().unwrap().into_struct_type().as_str())
                    .unwrap();

                let member_index = struct_type
                    .fields
                    .iter()
                    .position(|f| f.0 == ma.member)
                    .unwrap();

                self.builder
                    .build_struct_gep(
                        struct_value.into_pointer_value(),
                        member_index as u32,
                        "addrof_struct_member",
                    )
                    .unwrap()
                    .as_any_value_enum()
            }
            Expression::Null => self
                .context
                .i64_type()
                .ptr_type(AddressSpace::Generic)
                .const_zero()
                .as_any_value_enum(),
            Expression::ModuleAccess(_) => todo!(),
        }
    }

    fn visit_dereference_expression(&mut self, dereference: &DeReference) -> AnyValueEnum<'a> {
        let ptr = self.visit_expr(&dereference.identifier);

        if dereference.is_lvalue {
            ptr
        } else {
            self.builder
                .build_load(ptr.into_pointer_value(), "deref_ptr_address")
                .as_any_value_enum()
        }
    }

    fn visit_struct_literal(&mut self, struct_literal: &StructLiteral) -> AnyValueEnum<'a> {
        let struct_values: Vec<BasicValueEnum<'a>> = struct_literal
            .expressions
            .iter()
            .map(|expr| match self.visit_borrowed_expr(expr) {
                AnyValueEnum::ArrayValue(v) => v.as_basic_value_enum(),
                AnyValueEnum::IntValue(v) => v.as_basic_value_enum(),
                AnyValueEnum::FloatValue(v) => v.as_basic_value_enum(),
                AnyValueEnum::PhiValue(v) => v.as_basic_value(),
                AnyValueEnum::PointerValue(v) => v.as_basic_value_enum(),
                AnyValueEnum::StructValue(v) => v.as_basic_value_enum(),
                AnyValueEnum::VectorValue(v) => v.as_basic_value_enum(),
                _ => unreachable!("Garbage value in struct init!"),
            })
            .collect();

        self.context
            .const_struct(struct_values.as_slice(), /* packed: */ false)
            .as_any_value_enum()
    }

    fn visit_member_access(&mut self, member_access: &MemberAccess) -> AnyValueEnum<'a> {
        let struct_value = self.visit_expr(&member_access.object);

        // If the struct is loaded as pointer
        if struct_value.is_pointer_value() {
            let offset_ptr = self
                .get_struct_member_pointer_value(member_access, struct_value.into_pointer_value());
            self.builder
                .build_load(offset_ptr, "load_member_access")
                .as_any_value_enum()
        } else {
            // else use const extract value
            let struct_type = self
                .type_table
                .find_struct_type(
                    member_access
                        .ty
                        .as_ref()
                        .unwrap()
                        .into_struct_type()
                        .as_str(),
                )
                .unwrap();

            let member_index = struct_type
                .fields
                .iter()
                .position(|f| f.0 == member_access.member)
                .unwrap();

            struct_value
                .into_struct_value()
                .const_extract_value(&mut [member_index as u32])
                .as_any_value_enum()
        }
    }
}

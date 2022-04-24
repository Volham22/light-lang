use inkwell::values::{AnyValue, AnyValueEnum};
use inkwell::{FloatPredicate, IntPredicate};

use crate::generation::ir_generator::IRGenerator;
use crate::parser::visitors::{
    ArrayAccess, Binary, BinaryLogic, Call, ExpressionVisitor, Group, Literal, Unary,
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
                .bool_type()
                .const_int(*val as u64, false)
                .as_any_value_enum(),
            Literal::Identifier(name) => {
                let val_ptr = self.variables.get(name).unwrap();
                self.builder
                    .build_load(*val_ptr, name.as_str())
                    .as_any_value_enum()
            }
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
            None => panic!("wrong call"),
        }
    }

    fn visit_array_access(&mut self, call_expr: &ArrayAccess) -> AnyValueEnum<'a> {
        println!("access");
        let expr = self.visit_expr(&call_expr.index);
        let ptr = self.variables.get(&call_expr.identifier).unwrap();
        let value = self.get_int_value(expr);

        let offset_ptr = unsafe {
            self.builder
                .build_gep(*ptr, &[value], call_expr.identifier.as_str())
        };

        self.builder.build_load(offset_ptr, "load_array").into()
    }
}

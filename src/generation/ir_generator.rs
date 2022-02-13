use std::fmt::Display;

use crate::{
    parser::visitors::{Binary, BinaryLogic, Expression, ExpressionVisitor, Group, Literal, Unary},
    type_system::type_check::ValueType,
};

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    values::{AnyValue, AnyValueEnum, FloatValue, FunctionValue, IntValue},
    FloatPredicate, IntPredicate, OptimizationLevel,
};

struct IRGenerator<'a> {
    pub context: &'a Context, // LLVM Context
    pub builder: Builder<'a>,
    pub module: Module<'a>,
}

impl<'a> IRGenerator<'a> {
    pub fn generate_expression_ir(&mut self, expression: &Expression) -> ValueType {
        let function = self.generate_anonymous_function();
        let block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(block);

        let body = match expression {
            Expression::Literal(literal) => self.visit_literal(&literal),
            Expression::Binary(binary) => self.visit_binary(&binary),
            Expression::Group(group) => self.visit_group(&group),
            Expression::BinaryLogic(binary) => self.visit_binary_logic(&binary),
            Expression::Unary(unary) => self.visit_unary(&unary),
        };

        match body {
            AnyValueEnum::IntValue(_) => {
                self.builder.build_return(Some(&body.into_int_value()));
                ValueType::Number
            }
            AnyValueEnum::FloatValue(_) => {
                self.builder.build_return(Some(&body.into_float_value()));
                ValueType::Real
            }
            _ => todo!("Expression must return Real or Number!"),
        }
    }

    pub fn print_code(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }

    fn visit_expr(&mut self, expr: &Box<Expression>) -> AnyValueEnum<'a> {
        match &**expr {
            Expression::Literal(literal) => self.visit_literal(&literal),
            Expression::Binary(binary) => self.visit_binary(&binary),
            Expression::Group(group) => self.visit_group(&group),
            Expression::BinaryLogic(binary) => self.visit_binary_logic(&binary),
            Expression::Unary(unary) => self.visit_unary(&unary),
        }
    }

    fn get_int_value(&self, value: AnyValueEnum<'a>) -> IntValue<'a> {
        match value {
            AnyValueEnum::IntValue(value) => value,
            _ => panic!("Expected IntValue to unpack!"),
        }
    }

    fn get_float_value(&self, value: AnyValueEnum<'a>) -> FloatValue<'a> {
        match value {
            AnyValueEnum::FloatValue(value) => value,
            _ => panic!("Expected FloatValue to unpack!"),
        }
    }

    fn generate_anonymous_function(&mut self) -> FunctionValue<'a> {
        // void (void) function type
        let fntype = self.context.void_type().fn_type(&[], false);
        self.module
            .add_function("__anonymous_function", fntype, None)
    }
}

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
            Literal::Identifier(_) => todo!(),
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
}

unsafe fn execute_jit_function<'a, T: Display>(engine: &ExecutionEngine<'a>) {
    let fct = engine.get_function::<unsafe extern "C" fn() -> T>("__anonymous_function");

    match fct {
        Ok(f) => {
            let ret = f.call();
            println!("-> {}", ret);
        }
        Err(msg) => {
            println!("Execution failed: {}", msg);
        }
    }
}

pub fn generate_ir_code_jit(expression: &Expression) {
    let context = Context::create();

    let mut generator = IRGenerator {
        context: &context,
        builder: context.create_builder(),
        module: context.create_module("main"),
    };

    let global_type = generator.generate_expression_ir(expression);

    println!("========== Generated IR ==========");
    generator.print_code();
    println!("==================================");

    let engine = generator
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    // IR to native host code
    match global_type {
        ValueType::Number => unsafe { execute_jit_function::<i64>(&engine) },
        ValueType::Real => unsafe { execute_jit_function::<f64>(&engine) },
        ValueType::Bool => unsafe { execute_jit_function::<bool>(&engine) },
        ValueType::String => todo!("String handling"),
    };
}

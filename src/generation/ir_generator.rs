use crate::parser::visitors::{
    Binary, BinaryLogic, Expression, ExpressionVisitor, Group, Literal, Unary,
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{AnyValue, AnyValueEnum, FunctionValue, IntValue},
    IntPredicate, OptimizationLevel,
};

struct IRGenerator<'a> {
    pub context: &'a Context, // LLVM Context
    pub builder: Builder<'a>,
    pub module: Module<'a>,
}

impl<'a> IRGenerator<'a> {
    pub fn generate_expression_ir(&mut self, expression: &Expression) {
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

        self.builder.build_return(Some(&body.into_int_value()));
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

                self.builder
                    .build_int_add(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpadd",
                    )
                    .as_any_value_enum()
            }
            Binary::Minus(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_sub(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpadd",
                    )
                    .as_any_value_enum()
            }
            Binary::Multiply(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_mul(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpadd",
                    )
                    .as_any_value_enum()
            }
            Binary::Divide(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_signed_div(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpadd",
                    )
                    .as_any_value_enum()
            }
            Binary::Modulo(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_signed_rem(
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpadd",
                    )
                    .as_any_value_enum()
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

                self.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpieq",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::NotEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpineq",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::More(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_compare(
                        IntPredicate::SGT,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpimore",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::Less(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_compare(
                        IntPredicate::SLT,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpiless",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::MoreEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_compare(
                        IntPredicate::SGE,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmpimoreequal",
                    )
                    .as_any_value_enum()
            }
            BinaryLogic::LessEqual(l, r) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);

                self.builder
                    .build_int_compare(
                        IntPredicate::SLE,
                        self.get_int_value(left),
                        self.get_int_value(right),
                        "tmplessequal",
                    )
                    .as_any_value_enum()
            }
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> AnyValueEnum<'a> {
        match unary {
            Unary::Not(expr) => {
                let expr = self.visit_expr(expr);

                self.builder
                    .build_not(self.get_int_value(expr), "tmpnot")
                    .as_any_value_enum()
            }
            Unary::Negate(expr) => {
                let expr = self.visit_expr(expr);

                self.builder
                    .build_int_neg(self.get_int_value(expr), "tmpneg")
                    .as_any_value_enum()
            }
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

    generator.generate_expression_ir(expression);

    println!("========== Generated IR ==========");
    generator.print_code();
    println!("==================================");

    let engine = generator
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    let fct =
        unsafe { engine.get_function::<unsafe extern "C" fn() -> i64>("__anonymous_function") };

    // IR to native host code
    let compiled_fct = match fct {
        Ok(f) => f,
        Err(msg) => {
            println!("Execution failed: {}", msg);
            return;
        }
    };

    unsafe {
        let ret = compiled_fct.call();
        println!("-> {}", ret);
    };
}

use std::{collections::HashMap, fmt::Display};

use crate::{
    parser::visitors::{Expression, ExpressionVisitor, Statement},
    type_system::type_check::ValueType,
};

use crate::parser::visitors::StatementVisitor;

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    values::{AnyValueEnum, FloatValue, FunctionValue, IntValue, PointerValue},
    OptimizationLevel,
};

pub struct IRGenerator<'a> {
    pub context: &'a Context, // LLVM Context
    pub builder: Builder<'a>,
    pub module: Module<'a>,
    pub current_fn: Option<FunctionValue<'a>>,
    pub variables: HashMap<String, PointerValue<'a>>,
}

impl<'a> IRGenerator<'a> {
    pub fn generate_expression_ir(&mut self, stmt: &Statement) -> ValueType {
        self.generate_anonymous_function();
        let block = self
            .context
            .append_basic_block(self.current_fn.unwrap(), "entry");
        self.builder.position_at_end(block);

        let body = match stmt {
            Statement::Expression(expr) => self.visit_borrowed_expr(expr),
            Statement::VariableDeclaration(dec) => self.visit_declaration_statement(dec),
            Statement::VariableAssignment(ass_stmt) => self.visit_assignment_statement(ass_stmt),
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

    pub fn visit_expr(&mut self, expr: &Box<Expression>) -> AnyValueEnum<'a> {
        match &**expr {
            Expression::Literal(literal) => self.visit_literal(&literal),
            Expression::Binary(binary) => self.visit_binary(&binary),
            Expression::Group(group) => self.visit_group(&group),
            Expression::BinaryLogic(binary) => self.visit_binary_logic(&binary),
            Expression::Unary(unary) => self.visit_unary(&unary),
        }
    }

    pub fn visit_borrowed_expr(&mut self, expr: &Expression) -> AnyValueEnum<'a> {
        match expr {
            Expression::Literal(literal) => self.visit_literal(&literal),
            Expression::Binary(binary) => self.visit_binary(&binary),
            Expression::Group(group) => self.visit_group(&group),
            Expression::BinaryLogic(binary) => self.visit_binary_logic(&binary),
            Expression::Unary(unary) => self.visit_unary(&unary),
        }
    }

    pub fn get_int_value(&self, value: AnyValueEnum<'a>) -> IntValue<'a> {
        match value {
            AnyValueEnum::IntValue(value) => value,
            _ => panic!("Expected IntValue to unpack!"),
        }
    }

    pub fn get_float_value(&self, value: AnyValueEnum<'a>) -> FloatValue<'a> {
        match value {
            AnyValueEnum::FloatValue(value) => value,
            _ => panic!("Expected FloatValue to unpack!"),
        }
    }

    // allocate a value on the stack with a associated name and type,
    // in the entry block of the function
    pub fn create_entry_block_alloca(&self, name: &str, var_type: &ValueType) -> PointerValue<'a> {
        let builder = self.context.create_builder();

        let current_function = match self.current_fn {
            Some(f) => f,
            None => panic!("Trying to alloca a value on the global scope?!"),
        };

        let entry = current_function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        match var_type {
            ValueType::Number => self.builder.build_alloca(self.context.i64_type(), name),
            ValueType::Real => self.builder.build_alloca(self.context.f64_type(), name),
            ValueType::Bool => self.builder.build_alloca(self.context.bool_type(), name),
            ValueType::String => todo!("string support"),
        }
    }

    fn generate_anonymous_function(&mut self) {
        // void (void) function type
        let fntype = self.context.void_type().fn_type(&[], false);
        self.current_fn = Some(
            self.module
                .add_function("__anonymous_function", fntype, None),
        );
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

pub fn generate_ir_code_jit(expression: &Statement) {
    let context = Context::create();

    let mut generator = IRGenerator {
        context: &context,
        builder: context.create_builder(),
        module: context.create_module("main"),
        current_fn: None,
        variables: HashMap::new(),
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

use std::{collections::HashMap, ffi::CStr, fmt::Debug};

use crate::{
    parser::visitors::{Expression, ExpressionVisitor, Statement},
    type_system::value_type::ValueType,
};

use crate::parser::visitors::StatementVisitor;

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    values::{AnyValueEnum, FloatValue, FunctionValue, IntValue, PointerValue},
};

pub struct IRGenerator<'a> {
    pub context: &'a Context, // LLVM Context
    pub builder: Builder<'a>,
    pub module: Module<'a>,
    pub current_fn: Option<FunctionValue<'a>>,
    pub variables: HashMap<String, PointerValue<'a>>,
}

impl<'a> IRGenerator<'a> {
    pub fn generate_ir_anonymous(&mut self, stmt: &Statement) -> ValueType {
        self.generate_anonymous_function();
        let block = self
            .context
            .append_basic_block(self.current_fn.unwrap(), "entry");
        self.builder.position_at_end(block);

        let mut body: Option<AnyValueEnum> = None;
        match stmt {
            Statement::Expression(expr) => body = Some(self.visit_borrowed_expr(expr)),
            Statement::VariableDeclaration(dec) => {
                self.visit_declaration_statement(dec);
            }
            Statement::VariableAssignment(ass_stmt) => {
                self.visit_assignment_statement(ass_stmt);
            }
            Statement::Function(f) => {
                self.visit_function_statement(f);
            }
            Statement::Block(b) => {
                self.visit_block_statement(b);
            }
            Statement::Return(r) => {
                self.visit_return_statement(r);
            }
        };

        match body {
            Some(AnyValueEnum::IntValue(v)) => {
                self.builder.build_return(Some(&v));
                ValueType::Number
            }
            Some(AnyValueEnum::FloatValue(v)) => {
                self.builder.build_return(Some(&v));
                ValueType::Real
            }
            _ => {
                self.builder.build_return(None);
                ValueType::Void
            }
        }
    }

    pub fn generate_ir(&mut self, stmts: &Vec<Statement>) -> Option<ValueType> {
        for stmt in stmts {
            match stmt {
                Statement::Function(f) => {
                    self.visit_function_statement(f);
                }
                _ => {
                    return Some(self.generate_ir_anonymous(stmt));
                }
            }
        }

        None
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
            Expression::Call(_) => todo!(),
        }
    }

    pub fn visit_borrowed_expr(&mut self, expr: &Expression) -> AnyValueEnum<'a> {
        match expr {
            Expression::Literal(literal) => self.visit_literal(&literal),
            Expression::Binary(binary) => self.visit_binary(&binary),
            Expression::Group(group) => self.visit_group(&group),
            Expression::BinaryLogic(binary) => self.visit_binary_logic(&binary),
            Expression::Unary(unary) => self.visit_unary(&unary),
            Expression::Call(call) => self.visit_call(call),
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
            _ => todo!("type support"),
        }
    }

    pub fn visit_statement(&mut self, stmt: &Statement) -> Option<AnyValueEnum> {
        match stmt {
            Statement::Expression(expr) => Some(self.visit_borrowed_expr(expr)),
            Statement::Function(expr) => Some(self.visit_function_statement(expr)).unwrap(),
            Statement::Block(expr) => {
                self.visit_block_statement(expr);
                None
            }
            Statement::Return(expr) => {
                self.visit_return_statement(expr);
                None
            }
            Statement::VariableDeclaration(expr) => {
                self.visit_declaration_statement(expr);
                None
            }
            Statement::VariableAssignment(expr) => {
                self.visit_assignment_statement(expr);
                None
            }
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

unsafe fn execute_jit_function<'a, T: Debug>(engine: &ExecutionEngine<'a>) {
    let fct = engine.get_function::<unsafe extern "C" fn() -> T>("__anonymous_function");

    match fct {
        Ok(f) => {
            let ret = f.call();
            println!("-> {:?}", ret);
        }
        Err(msg) => {
            println!("Execution failed: {:?}", msg);
        }
    }
}

pub fn create_generator<'gen>(context: &'gen Context) -> IRGenerator<'gen> {
    IRGenerator {
        context: &context,
        builder: context.create_builder(),
        module: context.create_module("main"),
        current_fn: None,
        variables: HashMap::new(),
    }
}

pub fn generate_ir_code_jit(
    generator: &mut IRGenerator,
    engine: &ExecutionEngine,
    stmts: &Vec<Statement>,
) {
    let global_type = generator.generate_ir(stmts);

    println!("========== Generated IR ==========");
    generator.print_code();
    println!("==================================");

    // IR to native host code
    match global_type {
        Some(ValueType::Number) => unsafe { execute_jit_function::<i64>(&engine) },
        Some(ValueType::Real) => unsafe { execute_jit_function::<f64>(&engine) },
        Some(ValueType::Bool) => unsafe { execute_jit_function::<bool>(&engine) },
        Some(ValueType::Void) => unsafe { execute_jit_function::<()>(&engine) },
        Some(ValueType::String) => todo!("String handling"),
        Some(ValueType::Function) => {
            return;
        }
        _ => {
            return;
        }
    };
}
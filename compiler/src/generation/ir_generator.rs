use core::panic;
use std::{collections::HashMap, fmt::Debug};

use crate::{
    parser::visitors::{Expression, ExpressionVisitor, Statement},
    type_system::{types_table::TypeTable, value_type::ValueType},
};

use crate::parser::visitors::StatementVisitor;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{AnyTypeEnum, BasicType, BasicTypeEnum, PointerType, StructType},
    values::{AnyValueEnum, FloatValue, FunctionValue, IntValue, PointerValue},
    AddressSpace,
};

pub struct IRGenerator<'a> {
    pub module: Module<'a>,
    pub(super) type_table: TypeTable,
    pub(super) context: &'a Context, // LLVM Context
    pub(super) builder: Builder<'a>,
    pub(super) current_fn: Option<FunctionValue<'a>>,
    pub(super) variables: HashMap<String, PointerValue<'a>>,
    pub(super) struct_types: HashMap<String, StructType<'a>>,
    pub(super) loop_bb_stack: Vec<BasicBlock<'a>>,
    pub(super) has_branched: bool,
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
            Statement::IfStatement(if_stmt) => {
                self.visit_if_statement(if_stmt);
            }
            Statement::WhileStatement(while_stmt) => {
                self.visit_while_statement(while_stmt);
            }
            Statement::ForStatement(_) => unreachable!(),
            Statement::BreakStatement => unreachable!(),
            Statement::Struct(struct_stmt) => {
                self.visit_struct_statement(struct_stmt);
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
                Statement::Struct(s) => {
                    self.visit_struct_statement(s);
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
            Expression::Call(call) => self.visit_call(call),
            Expression::ArrayAccess(array) => self.visit_array_access(array),
            Expression::Null => self.visit_null_expression(),
            Expression::AddressOf(addr_of) => self.visit_address_of_expression(addr_of),
            Expression::DeReference(deref) => self.visit_dereference_expression(deref),
            Expression::MemberAccess(member_access) => self.visit_member_access(member_access),
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
            Expression::ArrayAccess(array) => self.visit_array_access(array),
            Expression::Null => self.visit_null_expression(),
            Expression::AddressOf(addr_of) => self.visit_address_of_expression(addr_of),
            Expression::DeReference(deref) => self.visit_dereference_expression(deref),
            Expression::MemberAccess(member_access) => self.visit_member_access(member_access),
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
            ValueType::String => self
                .builder
                .build_alloca(self.context.i8_type().ptr_type(AddressSpace::Generic), name),
            ValueType::Array(arr) => match *arr.array_type {
                ValueType::Array(_) => todo!(),
                ValueType::Number => self.builder.build_alloca(
                    self.context.i64_type().ptr_type(AddressSpace::Generic),
                    name,
                ),
                ValueType::Real => self.builder.build_alloca(
                    self.context.f64_type().ptr_type(AddressSpace::Generic),
                    name,
                ),
                ValueType::Bool => self.builder.build_alloca(
                    self.context.bool_type().ptr_type(AddressSpace::Generic),
                    name,
                ),
                ValueType::String => self
                    .builder
                    .build_alloca(self.context.i8_type().ptr_type(AddressSpace::Generic), name),
                ValueType::Function => todo!(),
                ValueType::Void => unreachable!(),
                ValueType::Pointer(_) => todo!(),
                ValueType::Null => todo!(),
                ValueType::Struct(_) => todo!(),
            },
            ValueType::Pointer(ptr_ty) => self
                .builder
                .build_alloca(self.get_ptr_type(&self.get_llvm_type(ptr_ty)), name),
            ValueType::Struct(s) => self.builder.build_alloca(
                self.struct_types.get(s).unwrap().as_basic_type_enum(),
                "struct_alloca",
            ),
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
            Statement::IfStatement(expr) => {
                self.visit_if_statement(expr);
                None
            }
            Statement::WhileStatement(expr) => {
                self.visit_while_statement(expr);
                None
            }
            Statement::ForStatement(_) => unreachable!(),
            Statement::BreakStatement => self.visit_break_statement(),
            Statement::Struct(struct_stmt) => self.visit_struct_statement(struct_stmt),
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

    pub fn get_ptr_type(&self, val: &AnyTypeEnum<'a>) -> PointerType<'a> {
        match val {
            AnyTypeEnum::ArrayType(t) => t.ptr_type(AddressSpace::Generic),
            AnyTypeEnum::IntType(t) => t.ptr_type(AddressSpace::Generic),
            AnyTypeEnum::FloatType(t) => t.ptr_type(AddressSpace::Generic),
            AnyTypeEnum::FunctionType(t) => t.ptr_type(AddressSpace::Generic),
            AnyTypeEnum::PointerType(t) => t.ptr_type(AddressSpace::Generic),
            AnyTypeEnum::VoidType(_) => self.context.i64_type().ptr_type(AddressSpace::Generic),
            _ => panic!("{:?}", val),
        }
    }

    pub fn block_has_branch(&mut self) -> bool {
        if self.has_branched {
            self.has_branched = false;
            true
        } else {
            false
        }
    }

    pub fn get_llvm_type(&self, value_type: &ValueType) -> AnyTypeEnum<'a> {
        match value_type {
            ValueType::Array(arr) => self.get_llvm_array_type(arr).into(),
            ValueType::Number => self.context.i64_type().into(),
            ValueType::Real => self.context.f64_type().into(),
            ValueType::Bool => self.context.bool_type().into(),
            ValueType::String => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Function => todo!("function pointer"),
            ValueType::Pointer(ptr) => match self.get_llvm_type(ptr) {
                AnyTypeEnum::ArrayType(arr) => arr.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::FloatType(f) => f.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::FunctionType(ft) => ft.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::IntType(i) => i.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::PointerType(pt) => pt.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::StructType(st) => st.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::VectorType(_) => unreachable!(),
                AnyTypeEnum::VoidType(_) => self
                    .context
                    .i64_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
            },
            ValueType::Void => self.context.void_type().into(),
            ValueType::Null => self
                .context
                .i64_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Struct(_) => todo!(),
        }
    }

    pub fn get_llvm_basic_type(&self, value_type: &ValueType) -> BasicTypeEnum<'a> {
        match value_type {
            ValueType::Array(arr) => self.get_llvm_array_type(arr).into(),
            ValueType::Number => self.context.i64_type().into(),
            ValueType::Real => self.context.f64_type().into(),
            ValueType::Bool => self.context.bool_type().into(),
            ValueType::String => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into(),
            ValueType::Function => todo!("function pointer"),
            ValueType::Pointer(ptr) => match self.get_llvm_type(ptr) {
                AnyTypeEnum::ArrayType(arr) => arr.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::FloatType(f) => f.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::FunctionType(ft) => ft.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::IntType(i) => i.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::PointerType(pt) => pt.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::StructType(st) => st.ptr_type(AddressSpace::Generic).into(),
                AnyTypeEnum::VectorType(_) => unreachable!(),
                AnyTypeEnum::VoidType(_) => self
                    .context
                    .i64_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
            },
            ValueType::Struct(_) => todo!(),
            _ => unreachable!("Building a struct of a forbidden type."),
        }
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

pub fn create_generator<'gen>(
    context: &'gen Context,
    name: &str,
    type_table: &TypeTable,
) -> IRGenerator<'gen> {
    IRGenerator {
        context: &context,
        type_table: type_table.clone(),
        builder: context.create_builder(),
        module: context.create_module(name),
        struct_types: HashMap::new(),
        current_fn: None,
        variables: HashMap::new(),
        loop_bb_stack: Vec::new(),
        has_branched: false,
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

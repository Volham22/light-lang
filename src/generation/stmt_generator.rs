use inkwell::{
    types::BasicMetadataTypeEnum,
    values::{AnyValueEnum, BasicValue, BasicValueEnum},
};

use crate::{
    parser::visitors::{
        BlockStatement, Expression, FunctionStatement, Literal, ReturnStatement, StatementVisitor,
        VariableAssignment, VariableDeclaration,
    },
    type_system::value_type::ValueType,
};

use super::ir_generator::IRGenerator;

impl<'a> IRGenerator<'a> {
    fn generate_block_instructions(&mut self, block: &BlockStatement) {
        for stmt in &block.statements {
            self.visit_statement(&stmt);
        }
    }
}

impl<'a> StatementVisitor<Option<AnyValueEnum<'a>>> for IRGenerator<'a> {
    fn visit_expression_statement(&mut self, expr: &Expression) -> Option<AnyValueEnum<'a>> {
        if let Expression::Literal(Literal::Identifier(name)) = expr {
            match self.variables.get(name) {
                Some(val) => self.builder.build_load(*val, name.as_str()),
                None => panic!("{} doest not exists in IR generation abort.", name),
            };
        }

        None
    }

    fn visit_declaration_statement(
        &mut self,
        var_dec: &VariableDeclaration,
    ) -> Option<AnyValueEnum<'a>> {
        let init_expr = self.visit_borrowed_expr(&var_dec.init_expr);
        let val_ptr =
            self.create_entry_block_alloca(var_dec.identifier.as_str(), &var_dec.variable_type);
        self.variables
            .insert(var_dec.identifier.to_string(), val_ptr);

        match init_expr {
            AnyValueEnum::IntValue(v) => self.builder.build_store(val_ptr, v),
            AnyValueEnum::FloatValue(v) => self.builder.build_store(val_ptr, v),
            _ => panic!(),
        };

        None
    }

    fn visit_assignment_statement(
        &mut self,
        var_ass: &VariableAssignment,
    ) -> Option<AnyValueEnum<'a>> {
        let new_expr = self.visit_borrowed_expr(&var_ass.new_value);
        let val_ptr = self.variables.get(&var_ass.identifier).unwrap();

        match new_expr {
            AnyValueEnum::IntValue(v) => self.builder.build_store(*val_ptr, v),
            AnyValueEnum::FloatValue(v) => self.builder.build_store(*val_ptr, v),
            _ => panic!(),
        };

        None
    }

    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> Option<AnyValueEnum<'a>> {
        let args_type = expr
            .args
            .as_ref()
            .unwrap()
            .iter()
            .map(|t| -> BasicMetadataTypeEnum {
                match t.1 {
                    ValueType::Number => self.context.i64_type().into(),
                    ValueType::Real => self.context.f64_type().into(),
                    ValueType::Bool => self.context.bool_type().into(),
                    ValueType::String => todo!(),
                    ValueType::Function => todo!(),
                    ValueType::Void => todo!(),
                }
            })
            .collect::<Vec<BasicMetadataTypeEnum>>();

        let fn_type = match expr.return_type {
            ValueType::Number => self.context.i64_type().fn_type(args_type.as_slice(), true),
            ValueType::Real => self.context.f64_type().fn_type(args_type.as_slice(), true),
            ValueType::Bool => self.context.bool_type().fn_type(args_type.as_slice(), true),
            _ => todo!(),
        };

        let fn_val = self
            .module
            .add_function(expr.callee.as_str(), fn_type, None);

        self.current_fn = Some(fn_val);
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
                _ => panic!(),
            }
        }

        self.generate_block_instructions(&expr.block);
        self.current_fn = None;

        fn_val.verify(true);

        Some(AnyValueEnum::FunctionValue(fn_val))
    }

    fn visit_block_statement(&mut self, expr: &BlockStatement) -> Option<AnyValueEnum<'a>> {
        let current_fn = self.current_fn.unwrap();
        let current_fn_bb = current_fn.get_last_basic_block().unwrap();

        let anonymous_block = self.context.append_basic_block(current_fn, "anon_block");
        self.builder.position_at_end(anonymous_block);

        for stmt in &expr.statements {
            self.visit_statement(&stmt);
        }

        self.builder.build_unconditional_branch(current_fn_bb);

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
}

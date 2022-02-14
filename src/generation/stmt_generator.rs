use inkwell::values::AnyValueEnum;

use crate::parser::visitors::{
    Expression, Literal, StatementVisitor, VariableAssignment, VariableDeclaration,
};

use super::ir_generator::IRGenerator;

impl<'a> StatementVisitor<AnyValueEnum<'a>> for IRGenerator<'a> {
    fn visit_expression_statement(&mut self, expr: &Expression) -> AnyValueEnum<'a> {
        if let Expression::Literal(Literal::Identifier(name)) = expr {
            match self.variables.get(name) {
                Some(val) => self.builder.build_load(*val, name.as_str()),
                None => panic!("{} doest not exists in IR generation abort.", name),
            };
        }

        self.visit_borrowed_expr(expr)
    }

    fn visit_declaration_statement(&mut self, var_dec: &VariableDeclaration) -> AnyValueEnum<'a> {
        let init_expr = self.visit_borrowed_expr(&var_dec.init_expr);
        let val_ptr =
            self.create_entry_block_alloca(var_dec.identifier.as_str(), &var_dec.variable_type);
        self.variables
            .insert(var_dec.identifier.to_string(), val_ptr);

        match init_expr {
            AnyValueEnum::IntValue(v) => self.builder.build_store(val_ptr, v),
            AnyValueEnum::FloatValue(v) => self.builder.build_store(val_ptr, v),
            _ => panic!()
        };

        init_expr
    }

    fn visit_assignment_statement(&mut self, var_ass: &VariableAssignment) -> AnyValueEnum<'a> {
        let new_expr = self.visit_borrowed_expr(&var_ass.new_value);
        let val_ptr = self.variables.get(&var_ass.identifier).unwrap();

        match new_expr {
            AnyValueEnum::IntValue(v) => self.builder.build_store(*val_ptr, v),
            AnyValueEnum::FloatValue(v) => self.builder.build_store(*val_ptr, v),
            _ => panic!(),
        };

        new_expr
    }
}

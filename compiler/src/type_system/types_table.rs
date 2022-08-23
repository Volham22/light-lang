use std::collections::HashMap;

use crate::parser::visitors::{FunctionStatement, StructStatement};

use super::value_type::ValueType;

#[derive(Clone)]
pub struct TypeTable {
    types: HashMap<String, ValueType>,
    struct_type: HashMap<String, StructStatement>,
    function_type: HashMap<String, FunctionStatement>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable {
            types: HashMap::new(),
            struct_type: HashMap::new(),
            function_type: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: &String, variable_type: &ValueType) {
        self.types.insert(name.to_string(), variable_type.clone());
    }

    pub fn add_struct_type(&mut self, struct_statement: &StructStatement) {
        self.struct_type.insert(
            struct_statement.type_name.to_string(),
            struct_statement.clone(),
        );
    }

    pub fn add_function_type(&mut self, function_statement: &FunctionStatement) {
        self.function_type.insert(
            function_statement.callee.clone(),
            function_statement.clone(),
        );
    }

    pub fn find_variable_type(&self, name: &str) -> Option<ValueType> {
        match self.types.get(name.into()) {
            Some(vt) => Some(vt.clone()),
            None => None,
        }
    }

    pub fn find_struct_type(&self, type_name: &str) -> Option<StructStatement> {
        match self.struct_type.get(type_name.into()) {
            Some(st) => Some(st.clone()),
            None => None,
        }
    }

    pub fn find_function_type(&self, function_name: &str) -> Option<FunctionStatement> {
        match self.function_type.get(function_name) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }
}

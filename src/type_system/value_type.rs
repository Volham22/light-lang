use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub enum ValueType {
    Number,
    Real,
    Bool,
    String,
    Function,
    Void,
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Number, ValueType::Number) => true,
            (ValueType::Real, ValueType::Real) => true,
            (ValueType::Bool, ValueType::Bool) => true,
            (ValueType::String, ValueType::String) => true,
            (ValueType::Function, ValueType::Function) => true,
            (ValueType::Void, ValueType::Void) => true,
            _ => false,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Number => print!("Number"),
            ValueType::Real => print!("Real"),
            ValueType::Bool => print!("Bool"),
            ValueType::String => print!("String"),
            ValueType::Function => print!("Function"),
            ValueType::Void => print!("Void"),
        };

        Ok(())
    }
}

impl Debug for ValueType {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("{}", self);
        Ok(())
    }
}

impl ValueType {
    pub fn is_compatible(ltype: ValueType, rtype: ValueType) -> bool {
        rtype == ltype
    }
}

impl FromStr for ValueType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "number" => Ok(ValueType::Number),
            "real" => Ok(ValueType::Real),
            "bool" => Ok(ValueType::Bool),
            "string" => Ok(ValueType::String),
            "void" => Ok(ValueType::Void),
            _ => Err("Unkown type"),
        }
    }
}

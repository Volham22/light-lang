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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Number => f.write_str("Number"),
            ValueType::Real => f.write_str("Real"),
            ValueType::Bool => f.write_str("Bool"),
            ValueType::String => f.write_str("String"),
            ValueType::Function => f.write_str("Function"),
            ValueType::Void => f.write_str("Void"),
        }
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

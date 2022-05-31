use std::{
    fmt::{Debug, Display},
    ops::Deref,
    str::FromStr,
};

#[derive(Clone)]
pub struct StaticArray {
    pub size: usize,
    pub array_type: Box<ValueType>,
}

#[derive(Clone)]
pub enum ValueType {
    Array(StaticArray),
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
            (ValueType::Function, ValueType::Function) => true, // TODO
            (ValueType::Void, ValueType::Void) => true,
            (ValueType::Array(lhs), ValueType::Array(rhs)) => {
                ValueType::is_compatible(lhs.array_type.deref(), rhs.array_type.deref())
                    && lhs.size == rhs.size
            }
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
            ValueType::Array(a) => {
                f.write_str("Array of ").unwrap();
                f.write_fmt(format_args!("{}", a.array_type.as_ref()))
                    .unwrap();
                f.write_fmt(format_args!(" size: {}", a.size))
            }
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
    pub fn is_compatible(ltype: &ValueType, rtype: &ValueType) -> bool {
        rtype == ltype
    }

    pub fn is_compatible_for_init(ltype: &ValueType, rtype: &ValueType) -> bool {
        match (ltype, rtype) {
            (ValueType::Array(lhs), rhs) => lhs.array_type.deref() == rhs,
            (lhs, ValueType::Array(rhs)) => lhs == rhs.array_type.deref(),
            _ => rtype == ltype,
        }
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

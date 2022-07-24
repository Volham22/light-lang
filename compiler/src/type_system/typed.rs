use crate::parser::visitors::{
    AddressOf, ArrayAccess, Binary, BinaryLogic, Call, DeReference, Expression, Group, Identifier,
    Literal, MemberAccess, StructLiteral, Unary,
};

use super::value_type::ValueType;

pub trait Typed {
    fn get_type(&self) -> ValueType;
    fn set_type(&mut self, new_type: ValueType);
}

impl Typed for i64 {
    fn get_type(&self) -> ValueType {
        ValueType::Number
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for f64 {
    fn get_type(&self) -> ValueType {
        ValueType::Real
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for bool {
    fn get_type(&self) -> ValueType {
        ValueType::Bool
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for String {
    fn get_type(&self) -> ValueType {
        ValueType::String
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for StructLiteral {
    fn get_type(&self) -> ValueType {
        self.literal_type.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.literal_type = Some(new_type);
    }
}

impl Typed for Identifier {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type)
    }
}

impl Typed for Literal {
    fn get_type(&self) -> ValueType {
        match self {
            Literal::Number(n) => n.get_type(),
            Literal::Real(r) => r.get_type(),
            Literal::Bool(b) => b.get_type(),
            Literal::StringLiteral(str_literal) => str_literal.get_type(),
            Literal::StructLiteral(struct_literal) => struct_literal.get_type(),
            Literal::Identifier(id) => id.get_type(),
        }
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for Binary {
    fn get_type(&self) -> ValueType {
        match self {
            Binary::Plus(l, _) => l.get_type(),
            Binary::Minus(l, _) => l.get_type(),
            Binary::Multiply(l, _) => l.get_type(),
            Binary::Divide(l, _) => l.get_type(),
            Binary::Modulo(l, _) => l.get_type(),
        }
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for Group {
    fn get_type(&self) -> ValueType {
        self.inner_expression.get_type()
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for BinaryLogic {
    fn get_type(&self) -> ValueType {
        match self {
            BinaryLogic::And(l, _) => l.get_type(),
            BinaryLogic::Or(l, _) => l.get_type(),
            BinaryLogic::Equal(l, _) => l.get_type(),
            BinaryLogic::NotEqual(l, _) => l.get_type(),
            BinaryLogic::More(l, _) => l.get_type(),
            BinaryLogic::Less(l, _) => l.get_type(),
            BinaryLogic::MoreEqual(l, _) => l.get_type(),
            BinaryLogic::LessEqual(l, _) => l.get_type(),
        }
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for Unary {
    fn get_type(&self) -> ValueType {
        match self {
            Unary::Not(e) => e.get_type(),
            Unary::Negate(e) => e.get_type(),
        }
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

impl Typed for Call {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type);
    }
}

impl Typed for ArrayAccess {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type);
    }
}

impl Typed for AddressOf {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type);
    }
}

impl Typed for DeReference {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type);
    }
}

impl Typed for MemberAccess {
    fn get_type(&self) -> ValueType {
        self.ty.as_ref().unwrap().clone()
    }

    fn set_type(&mut self, new_type: ValueType) {
        self.ty = Some(new_type);
    }
}

impl Typed for Expression {
    fn get_type(&self) -> ValueType {
        match self {
            Expression::Literal(l) => l.get_type(),
            Expression::Binary(b) => b.get_type(),
            Expression::Group(g) => g.get_type(),
            Expression::BinaryLogic(bl) => bl.get_type(),
            Expression::Unary(u) => u.get_type(),
            Expression::Call(c) => c.get_type(),
            Expression::ArrayAccess(aa) => aa.get_type(),
            Expression::AddressOf(ao) => ao.get_type(),
            Expression::DeReference(dr) => dr.get_type(),
            Expression::MemberAccess(ma) => ma.get_type(),
            Expression::Null => ValueType::Null,
        }
    }

    fn set_type(&mut self, _new_type: ValueType) {
        unreachable!()
    }
}

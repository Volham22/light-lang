pub enum Literal {
    Number(i64),
    Real(f64),
    Bool(bool),
    Identifier(String),
}

pub enum Binary {
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
}

pub struct Group {
    pub inner_expression: Box<Expression>,
}

pub enum BinaryLogic {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    More(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    MoreEqual(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
}

pub enum Unary {
    Not(Box<Expression>),
    Negate(Box<Expression>),
}

pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    Group(Group),
    BinaryLogic(BinaryLogic),
    Unary(Unary),
}

pub trait Visitor<T> {
    fn visit_expression(&mut self, expr: Expression) -> T;
}

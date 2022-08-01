use std::fmt::Display;

use crate::type_system::value_type::ValueType;

pub type Argument = (String, ValueType);

#[derive(Clone)]
pub enum Literal {
    Number(i64),
    Real(f64),
    Bool(bool),
    StringLiteral(String),
    StructLiteral(StructLiteral),
    Identifier(Identifier),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Literal::Number(n) => f.write_fmt(format_args!("{}", n)),
            Literal::Real(r) => f.write_fmt(format_args!("{}", r)),
            Literal::Bool(b) => f.write_fmt(format_args!("{}", b)),
            Literal::Identifier(s) => f.write_fmt(format_args!("{}", s)),
            Literal::StringLiteral(s) => f.write_fmt(format_args!("{}", s)),
            Literal::StructLiteral(_) => f.write_str("Struct init expr {...}"),
        }
    }
}

#[derive(Clone)]
pub struct Identifier {
    pub name: String,
    pub(crate) ty: Option<ValueType>,
    pub is_lvalue: bool,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Identifier; {}", self.name))
    }
}

#[derive(Clone)]
pub struct StructLiteral {
    pub type_name: String,
    pub expressions: Vec<Expression>,
    pub literal_type: Option<ValueType>,
}

#[derive(Clone)]
pub enum Binary {
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
}

#[derive(Clone)]
pub struct Group {
    pub inner_expression: Box<Expression>,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Unary {
    Not(Box<Expression>),
    Negate(Box<Expression>),
}

#[derive(Clone)]
pub struct Call {
    pub name: String,
    pub args: Option<Vec<Expression>>,
    pub ty: Option<ValueType>,
}

#[derive(Clone)]
pub struct ArrayAccess {
    pub identifier: Box<Expression>,
    pub index: Box<Expression>,
    pub ty: Option<ValueType>,
    pub is_lvalue: bool,
}

#[derive(Clone)]
pub struct AddressOf {
    pub identifier: Box<Expression>,
    pub ty: Option<ValueType>,
}

#[derive(Clone)]
pub struct DeReference {
    pub identifier: Box<Expression>,
    pub ty: Option<ValueType>,
    pub is_lvalue: bool,
}

#[derive(Clone)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: String,
    pub ty: Option<ValueType>,
}

#[derive(Clone)]
pub struct ModuleAccess {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone)]
pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    Group(Group),
    BinaryLogic(BinaryLogic),
    Unary(Unary),
    Call(Call),
    ArrayAccess(ArrayAccess),
    AddressOf(AddressOf),
    DeReference(DeReference),
    MemberAccess(MemberAccess),
    ModuleAccess(ModuleAccess),
    Null,
}

#[derive(Clone)]
pub struct VariableDeclaration {
    pub identifier: String,
    pub variable_type: ValueType,
    pub init_expr: Expression,
}

#[derive(Clone)]
pub struct VariableAssignment {
    pub identifier: Expression,
    pub new_value: Expression,
}

#[derive(Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Clone)]
pub struct FunctionStatement {
    pub callee: String,
    pub args: Option<Vec<Argument>>,
    // If the function has no block it means it's a declaration
    pub block: Option<BlockStatement>,
    pub return_type: ValueType,
    pub is_exported: bool,
}

pub type StructField = (String, ValueType);

#[derive(Clone)]
pub struct StructStatement {
    pub type_name: String,
    pub fields: Vec<StructField>,
    pub exported: bool,
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub expr: Expression,
}

#[derive(Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: BlockStatement,
    pub else_branch: Option<BlockStatement>,
}

#[derive(Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub loop_block: BlockStatement,
}

#[derive(Clone)]
pub struct ForStatement {
    pub init_expr: VariableDeclaration,
    pub loop_condition: Expression,
    pub next_expr: Box<Statement>,
    pub block_stmt: BlockStatement,
}

#[derive(Clone)]
pub struct ImportStatement {
    pub file_path: String,
    pub module_path: String,
}

#[derive(Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    VariableAssignment(VariableAssignment),
    Function(FunctionStatement),
    Struct(StructStatement),
    Block(BlockStatement),
    Return(ReturnStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    Import(ImportStatement),
    BreakStatement,
}

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&mut self, expr: &Expression) -> T;
    fn visit_declaration_statement(&mut self, expr: &VariableDeclaration) -> T;
    fn visit_assignment_statement(&mut self, expr: &VariableAssignment) -> T;
    fn visit_function_statement(&mut self, expr: &FunctionStatement) -> T;
    fn visit_struct_statement(&mut self, stct: &StructStatement) -> T;
    fn visit_block_statement(&mut self, expr: &BlockStatement) -> T;
    fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) -> T;
    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> T;
    fn visit_while_statement(&mut self, while_stmt: &WhileStatement) -> T;
    fn visit_for_statement(&mut self, for_stmt: &ForStatement) -> T;
    fn visit_break_statement(&mut self) -> T;
}

pub trait MutableStatementVisitor<T> {
    fn visit_expression_statement(&mut self, expr: &mut Expression) -> T;
    fn visit_declaration_statement(&mut self, expr: &mut VariableDeclaration) -> T;
    fn visit_assignment_statement(&mut self, expr: &mut VariableAssignment) -> T;
    fn visit_function_statement(&mut self, expr: &mut FunctionStatement) -> T;
    fn visit_struct_statement(&mut self, stct: &StructStatement) -> T;
    fn visit_block_statement(&mut self, expr: &mut BlockStatement) -> T;
    fn visit_return_statement(&mut self, return_stmt: &mut ReturnStatement) -> T;
    fn visit_if_statement(&mut self, if_stmt: &mut IfStatement) -> T;
    fn visit_while_statement(&mut self, while_stmt: &mut WhileStatement) -> T;
    fn visit_for_statement(&mut self, for_stmt: &mut ForStatement) -> T;
    fn visit_break_statement(&mut self) -> T;
}

pub trait ExpressionVisitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_group(&mut self, group: &Group) -> T;
    fn visit_binary_logic(&mut self, literal: &BinaryLogic) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_call(&mut self, call_expr: &Call) -> T;
    fn visit_array_access(&mut self, call_expr: &ArrayAccess) -> T;
    fn visit_null_expression(&mut self) -> T;
    fn visit_address_of_expression(&mut self, address_of: &AddressOf) -> T;
    fn visit_dereference_expression(&mut self, dereference: &DeReference) -> T;
    fn visit_struct_literal(&mut self, struct_literal: &StructLiteral) -> T;
    fn visit_member_access(&mut self, member_access: &MemberAccess) -> T;
}

pub trait MutableExpressionVisitor<T> {
    fn visit_literal(&mut self, literal: &mut Literal) -> T;
    fn visit_binary(&mut self, binary: &mut Binary) -> T;
    fn visit_group(&mut self, group: &mut Group) -> T;
    fn visit_binary_logic(&mut self, literal: &mut BinaryLogic) -> T;
    fn visit_unary(&mut self, unary: &mut Unary) -> T;
    fn visit_call(&mut self, call_expr: &mut Call) -> T;
    fn visit_array_access(&mut self, call_expr: &mut ArrayAccess) -> T;
    fn visit_null_expression(&mut self) -> T;
    fn visit_address_of_expression(&mut self, address_of: &mut AddressOf) -> T;
    fn visit_dereference_expression(&mut self, dereference: &mut DeReference) -> T;
    fn visit_struct_literal(&mut self, struct_literal: &mut StructLiteral) -> T;
    fn visit_member_access(&mut self, member_access: &mut MemberAccess) -> T;
}

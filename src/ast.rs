use super::token::{Token, Kind};

pub enum Op {
}

pub enum VarType {
    Normal,
    Array,
    Pointer,
}

struct Identifier {token: Token}
struct Number {token: Token}

pub enum Expression {
    Identifier(Identifier),
    Number(Number),
    BinaryOp(Box<Expression>, Op, Box<Expression>),
}
struct Declaration {vartype: VarType, ident: Identifier, expr: Expression}
struct Assignment {ident: Identifier, expr: Expression}
pub enum Statement {
    Declaration(Declaration),
    Assignment(Assignment)
}
use super::token::{Token};

#[derive(Debug)]
pub enum Op {
    Add(Token),
    Sub(Token),
    Mult(Token),
    Div(Token)
}

#[derive(Debug)]
pub enum VarType {
    Normal(Token),
    Array(Token),
    Pointer(Token),
}

#[derive(Debug)]
pub enum Expression {
    Number(u32, Token),
    Identifier(String, Token),
    BinaryOp(Box<Expression>, Op, Box<Expression>)
}

#[derive(Debug)]
pub enum Statement {
    Declaration {
        start: Token,
        vartype: VarType,
        name: String,
        expr: Expression,
    },
}
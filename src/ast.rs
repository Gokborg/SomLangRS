use super::token::{Token, Kind};

pub enum Op {
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
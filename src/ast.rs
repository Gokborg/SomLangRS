use crate::span::Span;

#[derive(Debug)]
pub enum Op {
    Add(Span),
    Sub(Span),
    Mult(Span),
    Div(Span)
}

#[derive(Debug)]
pub enum VarType {
    Normal(Span, String),
    Array(Span, Box<VarType>, Option<usize>),
    Pointer(Span, Box<VarType>),
    Func(Span, Box<VarType>, Vec<ArgDecl>)
}

#[derive(Debug)]
pub struct ArgDecl {
    span: Span,
    name: String,
    kind: VarType
}

#[derive(Debug)]
pub enum Expression {
    Number(Span, u32),
    Identifier(Span, String),
    BinaryOp(Span, Box<Expression>, Op, Box<Expression>)
}

#[derive(Debug)]
pub enum Statement {
    Declaration {
        span: Span,
        vartype: VarType,
        name: String,
        expr: Expression,
    },
    Assignment {
        span: Span,
        name: String,
        expr: Expression,
    },
}
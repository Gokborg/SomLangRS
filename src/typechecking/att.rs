use crate::{span::{Span, GetSpan}, ast::{Op}};

use super::scope::VarIndex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Void,
    UInt,
    Char,
    Bool,
    Pointer(Box<Type>),
    Array(Box<Type>),
    Infer
}

impl Type {
    pub fn infers(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Pointer(left), Self::Pointer(right)) => left.infers(right),
            (Self::Array(left), Self::Array(right)) => left.infers(right),
            (_, Self::Infer) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug)]

pub enum TStat {
    Decl{span: Span, varid: VarIndex, vartype: Type, expr: Option<TExpr>},
    Assignment{span: Span, target: TExpr, expr: TExpr},
    Expr{span: Span, expr: TExpr},
    Body{span: Span, content: Vec<TStat>},
    IfStatement{span: Span, cond: TExpr, body: Box<TStat>, child: Option<Box<TStat>>}
}

#[derive(Debug)]
pub enum TExpr {
    Uint{span: Span, value: u32},
    Var{span: Span, varid: VarIndex, vartype: Type},
    BinaryOp{span: Span, left: Box<TExpr>, op: Op, right: Box<TExpr>, vartype: Type},
}

impl TExpr {
    pub fn vartype(&self) -> &Type {
        match self {
            TExpr::Uint { .. } => &Type::UInt,
            TExpr::Var { vartype, .. } => vartype,
            TExpr::BinaryOp { vartype, .. } => vartype,
        }
    }
}

impl GetSpan for TExpr {
    fn span(&self) -> &Span {
        match self {
            TExpr::Uint { span, .. } => span,
            TExpr::Var { span, .. } => span,
            TExpr::BinaryOp { span, .. } => span,
        }
    }
}


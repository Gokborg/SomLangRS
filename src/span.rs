use crate::{token::Token, ast};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Loc,
    pub end: Loc,
}

impl Span {
    pub fn new(start: Loc, end: Loc) -> Self {
        Self { start, end }
    }

    pub fn from_token(start: &Token) -> Self {
        Self::from_tokens(start, start)
    }

    pub fn from_tokens(start: &Token, end: &Token) -> Self {
        Self {
            start: Loc {lineno: start.lineno, col: start.start as u32},
            end: Loc {lineno: end.lineno, col: (end.start + end.value.len()) as u32}
        }
    }
    pub fn start(&self) -> Loc {
        self.start
    }
    
    pub fn end(&self) -> Loc {
        self.end
    }
}

#[derive(Debug, Clone, Copy)]

pub struct Loc {
    pub lineno: u32,
    pub col: u32,
}

pub trait GetSpan {
    fn span(&self) -> &Span;
}

impl GetSpan for ast::Op {
    fn span(&self) -> &Span {
        match self {
            Self::Add(span) => span,
            Self::Sub(span) => span,
            Self::Mult(span) => span,
            Self::Div(span) => span,
            Self::CondEq(span) => span,
            Self::CondG(span) => span,
            Self::CondGEq(span) => span,
            Self::CondL(span) => span,
            Self::CondLEq(span) => span,
        }
    }
}

impl GetSpan for ast::VarType {
    fn span(&self) -> &Span {
        match self {
            Self::Normal(span, ..) => span,
            Self::Array(span, ..) => span,
            Self::Pointer(span, ..) => span,
            Self::Func(span, ..) => span,
        }
    }
}

impl GetSpan for ast::Expression {
    fn span(&self) -> &Span {
        match self {
            Self::Number(span, ..) => span,
            Self::Identifier(id, ..) => &id.span,
            Self::BinaryOp(span, ..) => span,
        }
    }
}

impl GetSpan for ast::Statement {
    fn span(&self) -> &Span {
        use ast::Statement::*;
        match self {
            Declaration { span, ..} => span,
            Assignment { span, ..} => span,
            Body { span, ..} => span,
            IfStatement { span, ..} => span,
            Expr { span, .. } => span,
        }
    }
}
use crate::{token::Token, ast};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: Loc,
    end: Loc,
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
            Self::Identifier(span, ..) => span,
            Self::BinaryOp(span, ..) => span,
        }
    }
}

impl GetSpan for ast::Statement {
    fn span(&self) -> &Span {
        match self {
            Self::Declaration { span, ..} => span,
            ast::Statement::Declaration { span, vartype, name, expr } => todo!(),
            ast::Statement::Assignment { span, name, expr } => todo!(),
        }
    }
}
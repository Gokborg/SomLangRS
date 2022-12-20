use crate::span::Span;
use crate::typechecking::att;

#[derive(Debug)]
pub struct ErrorContext {
    errors: Vec<Error>
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {errors: Vec::new()}
    }
    pub fn error(&mut self, kind: ErrorKind, span: Span) {
        self.errors.push(Error {kind, span})
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable,
    UndefinedType,
    UnexpectedToken,
    UnexpectedType{expected: att::Type}
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span
}
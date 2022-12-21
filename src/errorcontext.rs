use crate::span::Span;
use crate::typechecking::att;
use std::fmt::{self, Display, Write};

#[derive(Debug)]
pub struct ErrorContext<'a> {
    errors: Vec<Error>,
    lines: &'a [String]
}

impl <'a> ErrorContext<'a> {
    pub fn new(src: &'a [String]) -> Self {
        Self {errors: Vec::new(), lines: src}
    }
    pub fn error(&mut self, kind: ErrorKind, span: Span) {
        self.errors.push(Error {kind, span});
    }
    pub fn warn(&mut self, kind: ErrorKind, span: Span) {
        println!("{:?} {:?}", kind, span);        
    }
}

impl <'a> Display for ErrorContext<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = self.lines;
        for Error { kind, span: Span { start, end } } in self.errors.iter() {
            writeln!(fmt, "Error: {}", kind)?;
            let line = &lines[start.lineno as usize - 1];

            let width = if start.lineno == end.lineno {(end.col - start.col).max(1) as usize} else {line.len()};
            writeln!(fmt, "{}", line)?;
            writeln!(fmt, "{}{}", " ".repeat(start.col as usize), "^".repeat(width))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable,
    UndefinedType,
    UnexpectedToken,
    UnexpectedType{expected: att::Type, actual: att::Type},
    IgnoredResult,
    InvalidAssignTarget
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UndefinedVariable => write!(f, "Undefined variable"),
            ErrorKind::UndefinedType => write!(f, "Undefined type"),
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::UnexpectedType { expected, actual } => write!(f, "Expected type {:?} but got {:?}", expected, actual),
            ErrorKind::IgnoredResult => write!(f, "ignored expression result"),
            ErrorKind::InvalidAssignTarget => write!(f, "invalid assign target"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span
}

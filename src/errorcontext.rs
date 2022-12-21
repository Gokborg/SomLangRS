use crate::span::Span;
use crate::typechecking::att;
use std::fmt::{self, Display};
use crate::ansi;
use crate::token::Kind;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = self.lines;
        for Error { kind, span: Span { start, end } } in self.errors.iter() {
            writeln!(f, "{}", ansi::red(format!("Error: {}", kind)))?;
            if start.lineno == end.lineno {
                let line = &lines[start.lineno as usize - 1];
                let width = (end.col - start.col).max(1) as usize;
                let lineno = format!("{}", start.lineno);
                let bar = ansi::cyan(" |");
                writeln!(f, "{}{} {}", ansi::cyan(&lineno), bar, line)?;
                writeln!(f, "{}{} {}{}", " ".repeat(lineno.len()), bar, " ".repeat(start.col as usize), ansi::red("^".repeat(width)))?;
            } else {
                let line = &lines[start.lineno as usize - 1];
                let prefix_width = format!("{}", end.lineno).len();
                let lineno = format!("{}", start.lineno);
                let lineno = format!("{}{}", " ".repeat(prefix_width - lineno.len()), lineno);
                let bar = ansi::cyan(" |");
                writeln!(f, "{}{} {}", ansi::cyan(&lineno), bar, line)?;
                writeln!(f, "{}{} {}{}", " ".repeat(prefix_width), bar, ansi::red("_".repeat(start.col as usize)), ansi::red('^'))?;
                for lineno in start.lineno+1..end.lineno+1 {
                    let line = &lines[lineno as usize - 1];
                    let lineno = format!("{}", lineno);
                    let lineno = format!("{}{}", " ".repeat(prefix_width - lineno.len()), lineno);
                    writeln!(f, "{}{}{}{}", ansi::cyan(lineno), bar, ansi::red("|"), line)?;
                }
                writeln!(f, "{}{}{}{}{}", " ".repeat(prefix_width), bar, ansi::red("|"), ansi::red("_".repeat(end.col as usize - 1)), ansi::red('^'))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable,
    UndefinedType,
    UnexpectedToken{expected: Kind, actual: Kind},
    UnexpectedTokens{expected: &'static str, actual: Kind},
    UnexpectedType{expected: att::Type, actual: att::Type},
    IgnoredResult,
    InvalidAssignTarget
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UndefinedVariable => write!(f, "Undefined variable"),
            ErrorKind::UndefinedType => write!(f, "Undefined type"),
            ErrorKind::UnexpectedToken {expected, actual} => write!(f, "Expected token {:?} but got {:?}", expected, actual),
            ErrorKind::UnexpectedTokens {expected, actual} => write!(f, "Expected token {:} but got {:?}", expected, actual),
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

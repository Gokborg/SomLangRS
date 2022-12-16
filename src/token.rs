use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    NONE,
    WHITESPACE,
    NUMBER,
    IDENTIFIER,
    EQUAL,
    SEMICOLON,
    COLON,
    PLUS,
    MINUS,
    ASTERIK,
    SLASH,
    LET,

    CONDEQ,
    CONDG,
    CONDL,
    CONDGE,
    CONDLE,
}

#[derive(Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub lineno: u32,
    pub line: String,
    pub start: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}('{}' line-{} index-{})", self.kind, self.value, self.lineno, self.start)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}('{}' line-{} index-{})", self.kind, self.value, self.lineno, self.start)
    }
}
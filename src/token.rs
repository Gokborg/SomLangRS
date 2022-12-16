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
    LET,

    CONDEQ,
    CONDG,
    CONDL,
    CONDGE,
    CONDLE,
}

#[derive(Debug, Clone)]
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


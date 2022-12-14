#[derive(Debug, Clone)]
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


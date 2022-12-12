#[derive(Debug)]
pub enum Kind {
    WHITESPACE,
    NUMBER,
    IDENTIFIER,
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub lineno: u32,
    pub line: String,
    pub start: usize,
}


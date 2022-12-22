use std::fmt;

use crate::span::{Loc, Span};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum Kind {
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
    IF,
    ELIF,
    ELSE,
    OPENBRACE,
    CLOSEBRACE,
    OPENSQUARE,
    CLOSESQUARE,
    EOF,

    CONDEQ,
    CONDG,
    CONDL,
    CONDGE,
    CONDLE,
    Unknown
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub lineno: u32,
    pub start: usize,
}

impl Token {
    pub fn start_loc(&self) -> Loc {
        Loc { lineno: self.lineno, col: self.start as u32 }
    }
    pub fn end_loc(&self) -> Loc {
        Loc { lineno: self.lineno, col: (self.start + self.value.len()) as u32 }
    }
    pub fn span(&self) -> Span {
        Span::new(self.start_loc(), self.end_loc())
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}('{}' line-{} index-{})", self.kind, self.value, self.lineno, self.start)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}('{}' line-{} index-{})", self.kind, self.value, self.lineno, self.start);
    }
}
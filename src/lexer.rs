use std::str::Chars;

use super::token::{Token, Kind};
use wasm_bindgen::prelude::*;

pub struct Lexer<'a> {
    //Char buffer related things
    line: &'a str,
    chars: Chars<'a>,

    //Line related things
    tokens: Vec<Token>,
    lineno: u32,
}

#[wasm_bindgen]
pub struct TokenBuf {
    tokens: Vec<Token>
}
#[wasm_bindgen]
impl TokenBuf {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn kinds(&self) -> Vec<u32> {
        self.tokens.iter().map(|t| t.kind as u32).collect()
    }
    pub fn linenos(&self) -> Vec<usize> {
        self.tokens.iter().map(|t| t.lineno as usize).collect()
    }
    pub fn colnos(&self) -> Vec<usize> {
        self.tokens.iter().map(|t| t.start).collect()
    }
    pub fn lengths(&self) -> Vec<usize> {
        self.tokens.iter().map(|t| t.value.len()).collect()
    }
}
pub const EOF_CHAR: char = '\0';


#[wasm_bindgen]
pub fn lex(src: &str) -> TokenBuf {
    let tokens = Lexer::new().lex(src);
    TokenBuf::new(tokens)
} 

impl <'a> Lexer<'a> {
    pub fn new() -> Self {
        return Self {
            line: "",
            chars: "".chars(),
            tokens: Vec::new(),
            lineno: 0,
        }
    }
    
    pub fn lex(mut self, src: &'a str) -> Vec<Token> {
        for line in src.lines() {
            self.lineno += 1;
            self.line = line;
            self.chars = line.chars();
            while !self.done() {
                let start = self.pos();
                match self.next() {
                    '0'..='9' => {
                        self.next_while(|x| x.is_ascii_digit());
                        let end = self.pos();
                        self.tokens.push(Token{
                            kind: Kind::NUMBER,
                            value: self.line[start..end].to_string(),
                            lineno: self.lineno,
                            start: start
                        });
                    }
                    'a'..='z' | 'A'..='Z' => {
                        self.next_while(|x| x.is_ascii_alphanumeric());
                        let end = self.pos();
                        let kind = match &self.line[start..end] {
                            "if" => Kind::IF,
                            "elif" => Kind::ELIF,
                            "else" => Kind::ELSE,
                            "fn" => Kind::FN,
                            _ => Kind::IDENTIFIER
                        };
                        self.tokens.push(Token{
                            kind: kind,
                            value: self.line[start..end].to_string(),
                            lineno: self.lineno,
                            start: start
                        });
                    }
                    c => {
                        self.lex_symbol(start, c);
                    }
                }
            }
        }
        self.tokens.push(Token { kind: Kind::EOF, value: String::new(), lineno: self.lineno, start: self.pos()});
        return self.tokens;
    }

    #[inline]
    fn pos(&self) -> usize {
        self.line.len() - self.chars.as_str().len()
    }

    #[inline]
    fn done(&self) -> bool {
        self.chars.as_str().len() == 0
    }

    #[inline]
    fn has_next(&self) -> bool {
        !self.done()
    }

    #[inline]
    fn advance(&mut self) {
        self.chars.next();
    }

    #[inline]
    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    #[inline]
    fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    #[inline]
    fn next(&mut self) -> char {
        self.chars.next().unwrap_or(EOF_CHAR)
    }

    #[inline]
    fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> bool {
        if f(self.first()) {
            self.advance();
            true
        } else {
            false
        }
    }
    #[inline]
    fn next_if_eq(&mut self, c: char) -> bool {
        if self.first() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    #[inline]
    fn next_while<F: Fn(char) -> bool>(&mut self, f: F) {
        while f(self.first()) {
            self.advance();
        }
    }

    #[inline]
    fn lex_symbol(&mut self, start: usize, c: char) {
        let kind: Kind;
        match c {
            '{' => {kind = Kind::OPENBRACE}
            '}' => {kind = Kind::CLOSEBRACE}
            '(' => {kind = Kind::OPENPAREN}
            ')' => {kind = Kind::CLOSEPAREN}
            '[' => {kind = Kind::OPENSQUARE}
            ']' => {kind = Kind::CLOSESQUARE}
            ';' => {kind = Kind::SEMICOLON;}
            ':' => {kind = Kind::COLON;}
            ' ' | '\t' => {self.next_while(|c|c.is_whitespace()); kind = Kind::WHITESPACE;}
            '+' => {kind = Kind::PLUS;}
            '-' => {
                if self.next_if_eq('>') {
                    kind = Kind::ARROW;
                } else {
                    kind = Kind::MINUS;
                }
            }
            '*' => {kind = Kind::ASTERIK;}
            '/' => {kind = Kind::SLASH;}
            '=' => {
                if self.next_if_eq('=') {
                    kind = Kind::CONDEQ;
                }
                else {
                    kind = Kind::EQUAL;
                }
            }
            '>' => {
                if self.next_if_eq('=') {
                    kind = Kind::CONDGE;
                }
                else {
                    kind = Kind::CONDG;
                }
            }
            '<' => {
                if self.next_if_eq('=') {
                    kind = Kind::CONDLE;
                }
                else {
                    kind = Kind::CONDL;
                }
            }
            c => {
                if let Some(token) = self.tokens.last_mut() {
                    if token.kind == Kind::Unknown {
                        token.value.push(c);
                        return;
                    }
                }
                let end = self.pos();
                self.tokens.push(Token { kind: Kind::Unknown, value: self.line[start..end].to_string(), lineno: self.lineno, start });
                return;
            }
        }
        let end = self.pos();
        self.tokens.push(Token{
            kind: kind,
            value: self.line[start..end].to_string(),
            lineno: self.lineno,
            start: start
        });
    }
    
}

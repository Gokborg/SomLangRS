use super::token::{Token, Kind};
use wasm_bindgen::prelude::*;

pub struct Lexer {
    //Char buffer related things
    content: Vec<char>,
    pos: usize,

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



#[wasm_bindgen]
pub fn lex(src: &str) -> TokenBuf {
    let lines: Vec<String> = src.lines().map(|line|line.to_owned()).collect();
    let tokens = Lexer::new().lex(&lines);
    TokenBuf::new(tokens)
} 

impl Lexer {
    pub fn new() -> Self {
        return Self {
            content: Vec::new(),
            pos: 0,
            tokens: Vec::new(),
            lineno: 0,
        }
    }
    
    pub fn lex(mut self, lines: &[String]) -> Vec<Token> {
        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            self.lineno += 1;
            self.content = line.chars().collect();
            self.pos = 0;
            while !self.done() {
                match self.content[self.pos] {
                    '0'..='9' => {
                        self.next_while(Kind::NUMBER, |x| x.is_ascii_digit());
                    }
                    'a'..='z' | 'A'..='Z' => {
                        self.next_while(Kind::IDENTIFIER, |x| x.is_ascii_alphanumeric());
                    }
                    _ => {
                        self.lex_symbol();
                    }
                }
            }
        }
        self.tokens.push(Token { kind: Kind::EOF, value: String::new(), lineno: self.lineno, start: self.pos});
        return self.tokens;
    }

    #[inline]
    fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }

    #[inline]
    fn next(&mut self) -> char {
        if self.pos >= self.content.len() {
            return '\0';
        }
        self.pos += 1;
        if self.pos < self.content.len() {
            return self.content[self.pos];
        }
        return '\0';
    }

    #[inline]
    fn next_while<F: Fn(char) -> bool>(&mut self, mut kind: Kind, f: F) {
        let mut value: String = self.content.get(self.pos).unwrap().to_string();
        let start: usize = self.pos;
        while f(self.next()) {
            value.push(self.content[self.pos]);
        }
        match value.as_str() {
            "if" => kind = Kind::IF,
            "elif" => kind = Kind::ELIF,
            "else" => kind = Kind::ELSE,
            _ => {}
        }
        self.tokens.push(Token {
            kind: kind, 
            value: value,
            lineno: self.lineno,
            start: start,
        });
    }

    #[inline]
    fn lex_symbol(&mut self) {
        let start: usize = self.pos;
        let kind: Kind;
        let mut value: String = self.content[self.pos].to_string();
        match self.content[self.pos] {
            '{' => {kind = Kind::OPENBRACE}
            '}' => {kind = Kind::CLOSEBRACE}
            ';' => {kind = Kind::SEMICOLON;}
            ':' => {kind = Kind::COLON;}
            ' ' => {kind = Kind::WHITESPACE;}
            '+' => {kind = Kind::PLUS;}
            '-' => {kind = Kind::MINUS;}
            '*' => {kind = Kind::ASTERIK;}
            '/' => {kind = Kind::SLASH;}
            '=' => {
                if self.next() == '=' {
                    kind = Kind::CONDEQ;
                    value = "==".to_string();
                }
                else {
                    kind = Kind::EQUAL;
                }
            }
            '>' => {
                if self.next() == '=' {
                    kind = Kind::CONDGE;
                    value = ">=".to_string();
                }
                else {
                    kind = Kind::CONDG;
                }
            }
            '<' => {
                if self.next() == '=' {
                    kind = Kind::CONDLE;
                    value = "<=".to_string();
                }
                else {
                    kind = Kind::CONDL;
                }
            }
            _ => {
                self.pos += 1;
                if let Some(token) = self.tokens.last_mut() {
                    if token.kind == Kind::Unknown {
                        token.value.push(self.content[self.pos-1]);
                        return;
                    }
                }
                self.tokens.push(Token { kind: Kind::Unknown, value: self.content[self.pos-1].to_string(), lineno: self.lineno, start });
                return;
            }
        }
        self.pos += 1;
        self.tokens.push(Token{
            kind: kind,
            value: value,
            lineno: self.lineno,
            start: start
        });
    }
    
}

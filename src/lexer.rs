use super::token::{Token, Kind};

pub struct Lexer {
    //Char buffer related things
    content: Vec<char>,
    pos: usize,

    //Line related things
    tokens: Vec<Token>,
    lineno: u32,
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

    pub fn lex(&mut self, lines: Vec<String>) -> Vec<Token> {
        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            self.lineno += 1;
            self.content = line.chars().collect();
            while !self.done() {
                match self.content[self.pos] {
                    '0'..='9' => {
                        self.next_while(line, Kind::NUMBER, |x| x.is_ascii_digit());
                    }
                    'a'..='z' | 'A'..='Z' => {
                        self.next_while(line, Kind::IDENTIFIER, |x| x.is_ascii_alphanumeric());
                    }
                    _ => {
                        self.pos += 1;
                    }
                }
            }
        }
        return self.tokens.clone();
    }

    fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }

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

    fn next_while<F: Fn(char) -> bool>(&mut self, line: &String, kind: Kind, f: F) {
        let mut value: String = self.content.get(self.pos).unwrap().to_string();
        let start: usize = self.pos;
        while f(self.next()) {
            value.push(self.content[self.pos]);
        }
        self.tokens.push(Token {
            kind: kind, 
            value: value,
            lineno: self.lineno,
            line: line.clone(),
            start: start,
        });
    }
    
}

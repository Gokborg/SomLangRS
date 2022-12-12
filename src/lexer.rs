use super::token::{Token, Kind};

#[derive(Default)]
struct Buffer {
    current: char,
    pos: usize, 
    done: bool, 
    line: Vec<char>,
}

impl Buffer {
    fn set(&mut self, line: &str) {
        self.line = line.chars().collect();
        self.current = self.line[0];
        self.pos = 0;
        self.done = false;
    }

    fn next(&mut self) -> char {
        self.pos += 1;
        if self.pos < self.line.len() {
            self.current = self.line[self.pos];
        }
        else {
            self.done = true;
            self.current = '\0';
        }
        return self.current;
    }
}

pub fn lex(lines: Vec<String>) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: Buffer = Default::default();
    let mut lineno: u32 = 0;

    for line in lines.iter() {
        lineno += 1;
        buf.set(line);
        while !buf.done {
            match buf.current {
                '0'..='9' => {
                    let mut number: String = buf.current.to_string();
                    let start: usize = buf.pos;
                    while buf.next().is_ascii_digit() {
                        number.push(buf.current);
                    }
                    tokens.push(Token{ kind: Kind::NUMBER, value: number, lineno: lineno, line: line.clone(), start: start });
                }
                'a'..='z' | 'A'..='Z' => {
                    tokens.push(read_identifier(&mut buf, lineno, line));
                }
                ' ' => {
                    tokens.push(Token{ kind: Kind::WHITESPACE, value: " ".to_string(), lineno: lineno, line: line.clone(), start: buf.pos });
                    buf.next();
                }
                _ => {
                    buf.next();
                }
            }
        }
    }
    return tokens;
}

fn read_identifier(buf: &mut Buffer, lineno: u32, line: &String) -> Token {
    let start = buf.pos;
    let mut identifier: String = buf.current.to_string();
    while !buf.done && buf.next().is_alphanumeric() {
        identifier.push(buf.current);
    }
    return Token {
        kind: Kind::IDENTIFIER,
        value: identifier,
        lineno: lineno,
        line: line.clone(),
        start: buf.pos
    }
}
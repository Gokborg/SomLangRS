use super::token::{Token, Kind};
use super::buffer::{Buffer};

pub fn lex(lines: Vec<String>) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: Buffer<char> = Default::default();
    let mut lineno: u32 = 0;

    for line in lines.iter() {
        lineno += 1;
        buf.set(line.chars());
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
                '=' => {
                    tokens.push(Token{ kind: Kind::EQUAL, value: "=".to_string(), lineno: lineno, line: line.clone(), start: buf.pos });
                    buf.next();
                }
                ';' => {
                    tokens.push(Token{ kind: Kind::SEMICOLON, value: ";".to_string(), lineno: lineno, line: line.clone(), start: buf.pos });
                    buf.next();
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

fn read_identifier(buf: &mut Buffer<char>, lineno: u32, line: &String) -> Token {
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
use std::iter::Peekable;
use std::str::{Chars, Lines};

use super::token::{Token, Kind};
use super::buffer::{Buffer};

pub fn lex(lines: Vec<String>) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut lineno: u32 = 0;

    for line in lines.iter() {
        lineno += 1;
        let mut chars = line.chars().peekable();
        while let Some(current) = chars.peek() {
            match current {
                '0'..='9' => {
                    let mut number: String = current.to_string();
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

    let mut token_iter = tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        let state = token_iter;
        // parse 
        if (ifnoerror) {
            return declaration;
        }
        token_iter = state;
        // parse
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

pub struct Lexer<'a> {
    line: &'a str,
    chars: Peekable<Chars<'a>>,
    lineno: usize,
    colno: usize,
}

impl <'a> Lexer<'a> {

    fn new(line: &str, lineno: usize) -> Self {
        return Self { line, chars: line.chars().peekable(), lineno, colno: 1 };
    }
    
    fn next(&mut self) -> Option<char> {
        return self.chars.next();
    }

    fn peek(&mut self) -> Option<char> {
        return self.chars.peek().map(|c| *c);
    }
    
    //Gets the next char if the function passed returns true
    fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<char> {
        return self.chars.next_if(|x| f(*x));
    }

    //Keeps getting the next char while the function passed returns true
    fn next_while<F: Fn(char) -> bool>(&mut self, f: F) {
        while self.next_if(f).is_some() {}
    }

    fn create() {


        Token()
    }

}
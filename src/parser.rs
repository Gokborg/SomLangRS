use super::token::{Token, Kind};
use super::ast;
use super::parsers;

pub struct Parser <'a> {
    pub content: &'a [Token],
    pub pos: usize,
}

impl <'a> Parser <'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        return Parser {
            content: tokens,
            pos: 0
        }
    }

    pub fn parse(&mut self) -> Vec<ast::Statement> {
        let mut ast_nodes: Vec<ast::Statement> = Vec::new();
        while !self.done() {
            match self.content[self.pos].kind {
                Kind::LET => {
                    ast_nodes.push(parsers::decparser::parse_dec(self));
                }
                Kind::IDENTIFIER => {
                    ast_nodes.push(parsers::assignparser::parse_assign(self));
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
        return ast_nodes;
    }

    #[inline]
    pub fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }

    #[inline]
    pub fn current(&self) -> Token {
        return self.content[self.pos].clone();
    }

    pub fn next(&mut self) -> Option<Token> {
        self.pos += 1;
        if self.done() {
            return Option::None;
        }
        return Some(self.content[self.pos].clone());
    }

    pub fn expect(&mut self, kind: Kind) -> Token {
        let current: &Token = self.content.get(self.pos).unwrap();
        if kind == self.content[self.pos].kind {
            self.pos += 1;
            return current.clone();
        }
        else {
            println!("On line {}:", self.content[self.pos].lineno);
            //println!("\t{}", self.content[self.pos].line);
            println!("Expected '{:?}' got '{:?}' for {:?}", kind, self.content[self.pos].kind, self.content[self.pos].value);
            self.pos += 1;
            panic!();
        }
    }
}
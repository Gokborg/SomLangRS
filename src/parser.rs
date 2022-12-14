use super::token::{Token, Kind};

pub struct Parser <'a> {
    content: &'a [Token],
    pos: usize,
}

impl <'a> Parser <'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        return Parser {
            content: tokens,
            pos: 0
        }
    }

    pub fn parse(&mut self) {
        while !self.done() {
            match self.content[self.pos].kind {
                Kind::LET => {
                    self.expect(Kind::LET);
                    self.expect(Kind::IDENTIFIER);
                    self.expect(Kind::EQUAL);
                    println!("HERE");
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
    }

    #[inline]
    fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }

    fn expect(&mut self, kind: Kind) -> &Token {
        let current: &Token = self.content.get(self.pos).unwrap();
        if kind == self.content[self.pos].kind {
            self.pos += 1;
            return current;
        }
        else {
            println!("On line {}:", self.content[self.pos].lineno);
            println!("\t{}", self.content[self.pos].line);
            println!("\tExpected '{:?}' got '{:?}' for {:?}", kind, self.content[self.pos].kind, self.content[self.pos].value);
            self.pos += 1;
            panic!();
        }
    }
}
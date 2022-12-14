use std::collections::HashMap;

struct Parser {
    content: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.content = tokens;
        self.pos = 0;
        while !self.done() {
            match self.content[self.pos].kind {
                Kind::IDENTIFIER => {

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

    fn expect(kind: Kind) -> Token {

    }
}
use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::Span;

impl <'a> Parser<'a> {
    pub fn parse_body(&mut self) -> ast::Statement {
        //body opens with { closes with }
        let start: Token = self.expect(Kind::OPENBRACE);
        let mut content: Vec<Box<ast::Statement>> = Vec::new();
        while !self.done() && self.current().kind != Kind::CLOSEBRACE {
            if let Some(stmt) = self.parse_stmt() {
                content.push(Box::new(stmt));
            }
        }
        let span: Span = Span::from_tokens(&start, &self.current());
        self.expect(Kind::CLOSEBRACE);
        return ast::Statement::Body{span, content};
    }
}
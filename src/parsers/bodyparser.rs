use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::Span;

use super::PResult;

impl <'a> Parser<'a> {
    pub fn parse_body(&mut self) -> PResult<ast::Statement> {
        //body opens with { closes with }
        let start: Token = self.expect(Kind::OPENBRACE)?;
        let mut content: Vec<ast::Statement> = Vec::new();
        while !self.done() && self.current().kind != Kind::CLOSEBRACE {
            if let Ok(stmt) = self.parse_stmt() {
                content.push(stmt);
            }
        }
        let span: Span = Span::from_tokens(&start, &self.current());
        self.expect(Kind::CLOSEBRACE)?;
        return Ok(ast::Statement::Body{span, content});
    }
}
use crate::token::{Kind};
use crate::parser::{Parser};
use crate::span::Span;
use crate::ast;

impl <'a> Parser<'a> {
    pub fn parse_stmt(&mut self) -> Option<ast::Statement> {
        match self.current().kind {
            Kind::IDENTIFIER => {
                if self.peek().kind == Kind::COLON {
                    return Some(self.parse_dec())
                }
                else {
                    return Some(self.parse_assign());
                }
            }
            Kind::IF => {
                return Some(self.parse_if());
            }
            Kind::OPENBRACE => {
                let body = self.parse_body();
                return Some(body);
            }
            Kind::EOF => {
                self.next();
                return None;
            }
            _ => {
                let start = self.current();
                let expr = self.parse_expr();
                let end = self.expect(Kind::SEMICOLON);
                return Some(ast::Statement::Expr { span: Span::from_tokens(&start, &end), expr });
            }
        }
    }
}
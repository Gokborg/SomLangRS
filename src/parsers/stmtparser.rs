use crate::token::{Kind};
use crate::parser::{Parser};
use crate::span::Span;
use crate::ast;

use super::PResult;

impl <'a> Parser<'a> {
    pub fn parse_stmt(&mut self) -> PResult<ast::Statement> {
        match self.current().kind {
            Kind::IDENTIFIER => {
                if self.peek().kind == Kind::COLON {
                    return self.parse_dec()
                }
                else {
                    return self.parse_assign()
                }
            }
            Kind::IF => {
                return self.parse_if();
            }
            Kind::OPENBRACE => {
                let body = self.parse_body();
                return body;
            }
            Kind::EOF => {
                self.next();
                return Err(());
            }
            _ => {
                let start = self.current();
                let expr = self.parse_expr()?;
                let end = self.expect(Kind::SEMICOLON)?;
                return Ok(ast::Statement::Expr { span: Span::from_tokens(&start, &end), expr });
            }
        }
    }
}
use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span;

use super::PResult;

impl <'a> Parser<'a> {
    pub fn parse_dec(&mut self) -> PResult<ast::Statement> {
        let start: Token = self.expect(Kind::IDENTIFIER)?;
        self.expect(Kind::COLON)?;
        let mut vartype = None;
        match self.current().kind {
            Kind::IDENTIFIER => vartype = Some(self.parse_type()?),
            _ => {}
        }
        self.expect(Kind::EQUAL)?;
        let expr: ast::Expression = self.parse_expr()?;
        let span = span::Span::from_tokens(&start, &self.current());
        self.expect(Kind::SEMICOLON)?;
        return Ok(ast::Statement::Declaration {
            span: span, 
            target: ast::Identifier{span: span::Span::from_token(&start), name: start.value},
            vartype: vartype, 
            expr: Some(expr)
        });
    }
}
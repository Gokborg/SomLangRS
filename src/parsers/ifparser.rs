use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::{ast};
use crate::span::{Span};

use super::PResult;

impl <'a> Parser<'a> {
    pub fn parse_if(&mut self) -> PResult<ast::Statement> {
        let start: Token = self.current();
        self.next();
        let cond: ast::Expression = self.parse_expr()?;
        let body: ast::Statement = self.parse_body()?;
        let mut child: Option<ast::Statement> = Option::None;
        if self.current().kind == Kind::ELIF {
            child = Some(self.parse_if()?);
        }
        else if self.current().kind == Kind::ELSE {
            self.expect(Kind::ELSE);
            child = Some(self.parse_body()?);
        }
        let span = Span::from_tokens(&start, &self.current());

        return Ok(ast::Statement::IfStatement { 
            span: span, 
            cond: cond, 
            body: Box::new(body),
            child: Box::new(child)
        });
    }
}
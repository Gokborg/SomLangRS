use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::{Span};

use super::PResult;


impl <'a> Parser<'a> {
    pub fn parse_assign(&mut self) -> PResult<ast::Statement> {
        let start: Token = self.current();
        let target = self.parse_expr()?;
        match self.current().kind {
            Kind::SEMICOLON => {
                let semi = self.expect(Kind::SEMICOLON)?;
                return Ok(ast::Statement::Expr { span: Span::from_tokens(&start, &semi), expr: target });
            }
            Kind::EQUAL => {
                self.advance();
                let expr = self.parse_expr()?;

                let semi = self.expect(Kind::SEMICOLON)?;
                return Ok(ast::Statement::Assignment {
                    span: Span::from_tokens(&start, &semi), 
                    target, 
                    expr
                });
            }
            _ => {
                dbg!("TODO");
                Err(())
            }
        }
    }
}

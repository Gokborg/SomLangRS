use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::{Span};

impl <'a> Parser<'a> {
    pub fn parse_if(&mut self) -> ast::Statement {
        let start: Token = self.expect(Kind::IF);
        let cond: ast::Expression = self.parse_expr();
        let body: ast::Statement = self.parse_body();
        let child: Option<ast::Statement> = Option::None;
        let span = Span::from_tokens(&start, &self.current());

        return ast::Statement::IfStatement { 
            span: span, 
            cond: cond, 
            body: Box::new(body),
            child: Box::new(child)
        };
    }
}
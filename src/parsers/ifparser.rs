use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::{Span};

use super::{exprparser, bodyparser};

pub fn parse_if(parser: &mut Parser) -> ast::Statement {
    let start: Token = parser.expect(Kind::IF);
    let cond: ast::Expression = exprparser::parse_expr(parser);
    let body: ast::Statement = bodyparser::parse_body(parser);
    let child: Option<ast::Statement> = Option::None;
    let span = Span::from_tokens(&start, &parser.current());

    return ast::Statement::IfStatement { 
        span: span, 
        cond: cond, 
        body: Box::new(body),
        child: Box::new(child)
    };
}
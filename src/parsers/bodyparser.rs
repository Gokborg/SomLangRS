use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::Span;

use super::stmtparser;

pub fn parse_body(parser: &mut Parser) -> ast::Statement {
    //body opens with { closes with }
    let start: Token = parser.expect(Kind::OPENBRACE);
    let mut content: Vec<Box<ast::Statement>> = Vec::new();
    while !parser.done() && parser.current().kind != Kind::CLOSEBRACE {
        if let Some(stmt) = stmtparser::parse_stmt(parser) {
            content.push(Box::new(stmt));
        }
    }
    let span: Span = Span::from_tokens(&start, &parser.current());
    parser.expect(Kind::CLOSEBRACE);
    return ast::Statement::Body{span, content};
}
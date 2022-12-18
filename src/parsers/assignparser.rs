use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use super::exprparser;
use crate::span;

pub fn parse_assign(parser: &mut Parser) -> ast::Statement {
    let start: Token = parser.current();
    let varname: String = parser.expect(Kind::IDENTIFIER).value;
    parser.expect(Kind::EQUAL);
    let expr: ast::Expression = exprparser::parse_expr(parser);
    let span = span::Span::from_tokens(&start, &parser.current());
    parser.expect(Kind::SEMICOLON);
    return ast::Statement::Assignment {
        span: span, 
        name: varname, 
        expr: expr
    };
}
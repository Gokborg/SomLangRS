use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use super::exprparser;
use crate::span;

pub fn parse_dec(parser: &mut Parser) -> ast::Statement {
    let start: Token = parser.expect(Kind::LET);
    let varname: String = parser.expect(Kind::IDENTIFIER).value;
    parser.expect(Kind::COLON);
    let vartype_str = parser.expect(Kind::IDENTIFIER).value;
    parser.expect(Kind::EQUAL);
    let expr: ast::Expression = exprparser::parse_expr(parser);
    let span = span::Span::from_tokens(&start, &parser.current());
    parser.expect(Kind::SEMICOLON);
    let vartype = ast::VarType::Normal(span, vartype_str);
    return ast::Statement::Declaration {span: span, name: varname, vartype: vartype, expr: expr};
}
use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use super::exprparser;
pub fn parse_dec(parser: &mut Parser) -> ast::Statement {
    let start: Token = parser.expect(Kind::LET);
    let varname: String = parser.expect(Kind::IDENTIFIER).value;
    parser.expect(Kind::COLON);
    let vartype: ast::VarType = ast::VarType::Normal(parser.expect(Kind::IDENTIFIER));
    parser.expect(Kind::EQUAL);
    let expr: ast::Expression = exprparser::parse_expr(parser);
    parser.expect(Kind::SEMICOLON);
    return ast::Statement::Declaration {start: start, name: varname, vartype: vartype, expr: expr};
}
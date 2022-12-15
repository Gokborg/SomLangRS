use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;

pub fn parse_expr(parser: &mut Parser) -> ast::Expression {
    let current: Token = parser.content[parser.pos].clone();
    match current.kind {
        Kind::NUMBER => {
            parser.pos += 1;
            return ast::Expression::Number(current.value.parse::<u32>().unwrap(), current)
        },
        Kind::IDENTIFIER => {
            parser.pos += 1;
            return ast::Expression::Identifier(current.value.clone(), current);
        }
        _ => {
            panic!("Failed to parse expression");
        }
    }
}
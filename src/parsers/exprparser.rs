use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;

pub fn parse_expr(parser: &mut Parser) -> ast::Expression {
    return parse_expr_l3(parser);
}

fn parse_expr_l3(parser: &mut Parser) -> ast::Expression {
    return generic_parse_binop(
        parser, 
        parse_expr_l2,
        &[Kind::PLUS, Kind::MINUS]
    );
}

fn parse_expr_l2(parser: &mut Parser) -> ast::Expression {
    return generic_parse_binop(
        parser, 
        parse_expr_l1,
        &[Kind::ASTERIK, Kind::SLASH]
    );
}

fn parse_expr_l1(parser: &mut Parser) -> ast::Expression {
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

fn generic_parse_binop<F: Fn(&mut Parser) -> ast::Expression>(parser: &mut Parser, f: F, kinds: &[Kind]) -> ast::Expression {
    let mut expr1: ast::Expression = f(parser);
    while kinds.contains(&parser.current().kind) {
        let op: ast::Op;
        match parser.current().kind {
            Kind::PLUS => {
                op = ast::Op::Add(parser.current());
            }
            Kind::MINUS => {
                op = ast::Op::Sub(parser.current());
            }
            Kind::ASTERIK => {
                op = ast::Op::Mult(parser.current());
            }
            Kind::SLASH => {
                op = ast::Op::Div(parser.current());
            }
            _ => {
                panic!("Unknown operator: {}", parser.current());
            }
        }
        parser.next();
        let expr2: ast::Expression = f(parser);
        expr1 = ast::Expression::BinaryOp(Box::new(expr1), op, Box::new(expr2))
    }
    return expr1;
}
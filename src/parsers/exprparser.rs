use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span;

pub fn parse_expr(parser: &mut Parser) -> ast::Expression {
    return parse_expr_l4(parser);
}

fn parse_expr_l4(parser: &mut Parser) -> ast::Expression {
    return generic_parse_binop(
        parser, 
        parse_expr_l3,
        &[Kind::CONDEQ, Kind::CONDG, Kind::CONDGE, Kind::CONDL, Kind::CONDLE]
    );
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
            return ast::Expression::Number(span::Span::from_token(&current), current.value.parse::<u32>().unwrap())
        },
        Kind::IDENTIFIER => {
            parser.pos += 1;
            return ast::Expression::Identifier(span::Span::from_token(&current), current.value.clone());
        }
        _ => {
            panic!("Failed to parse expression");
        }
    }
}

fn generic_parse_binop<F: Fn(&mut Parser) -> ast::Expression>(parser: &mut Parser, f: F, kinds: &[Kind]) -> ast::Expression {
    let start_tok: &Token = &parser.current();
    let mut expr1: ast::Expression = f(parser);
    while kinds.contains(&parser.current().kind) {
        let op: ast::Op;
        let op_span = span::Span::from_token(&parser.current());
        match parser.current().kind {
            Kind::PLUS => {
                op = ast::Op::Add(op_span);
            }
            Kind::MINUS => {
                op = ast::Op::Sub(op_span);
            }
            Kind::ASTERIK => {
                op = ast::Op::Mult(op_span);
            }
            Kind::SLASH => {
                op = ast::Op::Div(op_span);
            }
            Kind::CONDEQ => {
                op = ast::Op::CondEq(op_span);
            }
            Kind::CONDG => {
                op = ast::Op::CondG(op_span);
            }
            Kind::CONDGE => {
                op = ast::Op::CondGEq(op_span);
            }
            Kind::CONDL => {
                op = ast::Op::CondL(op_span);
            }
            Kind::CONDLE => {
                op = ast::Op::CondLEq(op_span);
            }
            _ => {
                panic!("Unknown operator: {}", parser.current());
            }
        }
        parser.next();
        let expr2: ast::Expression = f(parser);
        let end_tok: &Token = &parser.current();
        let bin_span = span::Span::from_tokens(start_tok, end_tok);
        expr1 = ast::Expression::BinaryOp(bin_span, Box::new(expr1), op, Box::new(expr2));
    }
    return expr1;
}
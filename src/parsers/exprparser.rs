use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span;

impl <'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> ast::Expression {
        return self.parse_expr_l4();
    }

    fn parse_expr_l4(&mut self) -> ast::Expression {
        return self.generic_parse_binop(
            Parser::parse_expr_l3,
            &[Kind::CONDEQ, Kind::CONDG, Kind::CONDGE, Kind::CONDL, Kind::CONDLE]
        );
    }
    fn parse_expr_l3(&mut self) -> ast::Expression {
        return self.generic_parse_binop(
            Parser::parse_expr_l2,
            &[Kind::PLUS, Kind::MINUS]
        );
    }

    fn parse_expr_l2(&mut self) -> ast::Expression {
        return self.generic_parse_binop(
            Parser::parse_expr_l1,
            &[Kind::ASTERIK, Kind::SLASH]
        );
    }

    fn parse_expr_l1(&mut self) -> ast::Expression {
        let current: Token = self.content[self.pos].clone();
        match current.kind {
            Kind::NUMBER => {
                self.pos += 1;
                return ast::Expression::Number(span::Span::from_token(&current), current.value.parse::<u32>().unwrap())
            },
            Kind::IDENTIFIER => {
                self.pos += 1;
                return ast::Expression::Identifier(ast::Identifier{span: span::Span::from_token(&current), name: current.value.clone()});
            }
            _ => {
                panic!("Failed to parse expression on token: {}", current);
            }
        }
    }

    fn generic_parse_binop<F: Fn(&mut Self) -> ast::Expression>(&mut self, f: F, kinds: &[Kind]) -> ast::Expression {
        let start_tok: &Token = &self.current();
        let mut expr1: ast::Expression = f(self);
        while kinds.contains(&self.current().kind) {
            let op: ast::Op;
            let op_span = span::Span::from_token(&self.current());
            match self.current().kind {
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
                    panic!("Unknown operator: {}", self.current());
                }
            }
            self.next();
            let expr2: ast::Expression = f(self);
            let end_tok: &Token = &self.current();
            let bin_span = span::Span::from_tokens(start_tok, end_tok);
            expr1 = ast::Expression::BinaryOp(bin_span, Box::new(expr1), op, Box::new(expr2));
        }
        return expr1;
    }
}
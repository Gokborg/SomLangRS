use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span::{self, GetSpan};
use crate::errorcontext::ErrorKind;

use super::PResult;

impl <'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> PResult<ast::Expression> {
        return self.parse_expr_l4();
    }

    fn parse_expr_l4(&mut self) -> PResult<ast::Expression> {
        return self.generic_parse_binop(
            Parser::parse_expr_l3,
            |token| {Some(match token.kind {
                Kind::CONDEQ => ast::Op::CondEq(token.span()),
                Kind::CONDG => ast::Op::CondG(token.span()),
                Kind::CONDGE => ast::Op::CondGEq(token.span()),
                Kind::CONDL => ast::Op::CondL(token.span()),
                Kind::CONDLE => ast::Op::CondLEq(token.span()),
                _ => return None
            })}
        );
    }
    fn parse_expr_l3(&mut self) -> PResult<ast::Expression> {
        return self.generic_parse_binop(
            Parser::parse_expr_l2,
            |token| {Some(match token.kind {
                Kind::PLUS => ast::Op::Add(token.span()),
                Kind::MINUS => ast::Op::Sub(token.span()),
                _ => return None
            })}
        );
    }

    fn parse_expr_l2(&mut self) -> PResult<ast::Expression> {
        return self.generic_parse_binop(
            Parser::parse_expr_l1,
            |token| {Some(match token.kind {
                Kind::ASTERIK => ast::Op::Mult(token.span()),
                Kind::SLASH => ast::Op::Div(token.span()),
                _ => return None
            })}
        );
    }

    fn parse_expr_l1(&mut self) -> PResult<ast::Expression> {
        let current: Token = self.content[self.pos].clone();
        match current.kind {
            Kind::NUMBER => {
                self.pos += 1;
                return Ok(ast::Expression::Number(span::Span::from_token(&current), current.value.parse::<u32>().unwrap()))
            },
            Kind::IDENTIFIER => {
                self.pos += 1;
                return Ok(ast::Expression::Identifier(ast::Identifier{span: span::Span::from_token(&current), name: current.value.clone()}));
            }
            _ => {
                self.advance();
                self.err.error(ErrorKind::UnexpectedTokens { expected: "NUMBER or IDENTIFIER", actual: current.kind.clone() }, current.span());
                return PResult::Err(());
            }
        }
    }

    fn generic_parse_binop<F: Fn(&mut Self) -> PResult<ast::Expression>, OpF: Fn(&Token) -> Option<ast::Op>>(&mut self, f: F, kinds: OpF) -> PResult<ast::Expression> {
        let start_tok: &Token = &self.current();
        let mut expr1: ast::Expression = f(self)?;
        while let Some(op) = kinds(&self.current()) {
            self.next();
            let expr2: ast::Expression = f(self)?;
            let bin_span = span::Span::new(start_tok.start_loc(), expr2.span().end());
            expr1 = ast::Expression::BinaryOp(bin_span, Box::new(expr1), op, Box::new(expr2));
        }
        return Ok(expr1);
    }
}
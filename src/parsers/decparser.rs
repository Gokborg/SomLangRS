use crate::token::{Token, Kind};
use crate::parser::{Parser};
use crate::ast;
use crate::span;

impl <'a> Parser<'a> {
    pub fn parse_dec(&mut self) -> ast::Statement {
        let start: Token = self.expect(Kind::IDENTIFIER);
        self.expect(Kind::COLON);
        let vartype_str = self.expect(Kind::IDENTIFIER).value;
        self.expect(Kind::EQUAL);
        let expr: ast::Expression = self.parse_expr();
        let span = span::Span::from_tokens(&start, &self.current());
        self.expect(Kind::SEMICOLON);
        let vartype = ast::VarType::Normal(span, vartype_str);
        return ast::Statement::Declaration {
            span: span, 
            target: ast::Identifier{span: span::Span::from_token(&start), name: start.value},
            vartype: vartype, 
            expr: expr
        };
    }
}
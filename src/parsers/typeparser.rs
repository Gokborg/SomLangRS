use crate::{parser::Parser, token::Kind, ast::VarType, span::Span};

use super::PResult;

impl <'a> Parser <'a> {
    pub fn parse_type(&mut self) -> PResult<VarType> {
        let ident =  self.expect(Kind::IDENTIFIER)?;
        let mut node = VarType::Normal(Span::from_token(&ident), ident.value.clone());

        loop {match self.current().kind {
            Kind::ASTERIK => {
                self.advance();
                node = VarType::Pointer(Span::from_tokens(&ident, &self.current()), Box::new(node));
            },
            Kind::OPENSQUARE => {
                self.advance();
                if self.current().kind == Kind::CLOSESQUARE {
                    node = VarType::Array(Span::from_token(&self.current()), Box::new(node), None);
                } else {
                    let index = self.parse_expr()?;
                    node = VarType::Array(Span::from_token(&self.current()), Box::new(node), Some(index));
                }
                node = VarType::Pointer(Span::from_tokens(&ident, &self.current()), Box::new(node));
            },
            // TODO: parse function pointer type
            _ => {break;},
        }}
        Ok(node)
    }
}
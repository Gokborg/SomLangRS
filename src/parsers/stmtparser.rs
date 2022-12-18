use crate::token::{Kind};
use crate::parser::{Parser};
use crate::ast;

use super::{decparser, assignparser, bodyparser, ifparser};

pub fn parse_stmt(parser: &mut Parser) -> Option<ast::Statement> {
    match parser.current().kind {
        Kind::IDENTIFIER => {
            if(parser.peek().kind == Kind::COLON) {
                return Some(decparser::parse_dec(parser))
            }
            else {
                return Some(assignparser::parse_assign(parser));
            }

        }
        Kind::IF => {
            return Some(ifparser::parse_if(parser));
        }
        Kind::OPENBRACE => {
            let body = bodyparser::parse_body(parser);
            return Some(body);
        }
        Kind::EOF => {
            parser.next();
            return None;
        }
        _ => {
            println!("{:?}", parser.current().kind);
            panic!("What have you done.");
        }
    }
}
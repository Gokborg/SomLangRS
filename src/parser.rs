use super::token::{Token, Kind};
use super::ast;

pub struct Parser <'a> {
    content: &'a [Token],
    pos: usize,
}

impl <'a> Parser <'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        return Parser {
            content: tokens,
            pos: 0
        }
    }

    pub fn parse(&mut self) -> Vec<ast::Statement> {
        let mut ast_nodes: Vec<ast::Statement> = Vec::new();
        while !self.done() {
            match self.content[self.pos].kind {
                Kind::LET => {
                    let start: Token = self.expect(Kind::LET);
                    let varname: String = self.expect(Kind::IDENTIFIER).value;
                    self.expect(Kind::COLON);
                    let vartype: ast::VarType = ast::VarType::Normal(self.expect(Kind::IDENTIFIER));
                    self.expect(Kind::EQUAL);
                    let expr: ast::Expression = self.parse_expr();
                    ast_nodes.push(ast::Statement::Declaration {start: start, name: varname, vartype: vartype, expr: expr});
                    self.expect(Kind::SEMICOLON);
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
        return ast_nodes;
    }

    fn parse_expr(&mut self) -> ast::Expression {
        let current: Token = self.content[self.pos].clone();
        match current.kind {
            Kind::NUMBER => {
                self.pos += 1;
                return ast::Expression::Number(current.value.parse::<u32>().unwrap(), current)
            },
            Kind::IDENTIFIER => {
                self.pos += 1;
                return ast::Expression::Identifier(current.value.clone(), current);
            }
            _ => {
                panic!("Failed to parse expression");
            }
        }
    }

    #[inline]
    fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }

    fn expect(&mut self, kind: Kind) -> Token {
        let current: &Token = self.content.get(self.pos).unwrap();
        if kind == self.content[self.pos].kind {
            self.pos += 1;
            return current.clone();
        }
        else {
            println!("On line {}:", self.content[self.pos].lineno);
            println!("\t{}", self.content[self.pos].line);
            println!("\tExpected '{:?}' got '{:?}' for {:?}", kind, self.content[self.pos].kind, self.content[self.pos].value);
            self.pos += 1;
            panic!();
        }
    }
}
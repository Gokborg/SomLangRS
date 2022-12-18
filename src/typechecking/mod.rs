mod scope;
mod types;

use crate::{ast, errorcontext::ErrorContext};

#[derive(Debug)]
pub struct TypeChecker<'a> {
    err: &'a mut ErrorContext
}
impl <'a> TypeChecker<'a> {
    pub fn check(err: &'a mut ErrorContext, statements: &[ast::Statement]) -> Self {
        let mut checker = Self { err };
        checker.check_body(statements);
        checker
    }

    fn check_body(&mut self, statements: &[ast::Statement]) {
        for statement in statements {
            self.check_statement(statement);
        }
    }

    fn check_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Declaration { span, vartype, target: name, expr } => todo!(),
            ast::Statement::Body { content, span } => todo!(),
            ast::Statement::IfStatement { cond, body, child, span } => todo!(),
            ast::Statement::Assignment { span, target, expr } => todo!(),
            ast::Statement::Expr { span, expr } => todo!(),
        }
    }
}
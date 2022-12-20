mod scope;
pub mod att;

use crate::{ast, errorcontext::{ErrorContext, ErrorKind}, parsers::PResult, span::GetSpan};

use self::scope::Scopes;

#[derive(Debug)]
pub struct TypeChecker<'a> {
    pub err: &'a mut ErrorContext,
    pub scopes: Scopes,
}
impl <'a> TypeChecker<'a> {
    pub fn check(err: &'a mut ErrorContext, statements: &[ast::Statement]) -> (Self, Vec<att::TStat>) {
        let mut checker = Self { err, scopes: Scopes::new() };
        let att = checker.check_body(statements);
        (checker, att)
    }

    fn check_body(&mut self, statements: &[ast::Statement]) -> Vec<att::TStat> {
        let mut stats = Vec::new();
        for statement in statements {
            stats.push(self.check_statement(statement));
        }
        stats
    }

    fn check_statement(&mut self, statement: &ast::Statement) -> att::TStat {
        match statement {
            ast::Statement::Declaration { span, vartype, target: name, expr } => {
                let mut vartype = vartype.as_ref().map_or(att::Type::Infer, |vartype| self.check_type(vartype));
                let output = if let Some(expr) = expr {
                    let expr = self.check_expr(expr);
                    let exprtype = expr.vartype().clone();
                    expr.span();
                    if exprtype.infers(&vartype) {
                        vartype = exprtype;
                    } else {
                        self.err.error(ErrorKind::UnexpectedType{expected: vartype.clone()}, *expr.span() )
                    }
                    att::TStat::Decl { span: span.clone(), vartype: vartype.clone(), expr: Some(expr) }
                } else {
                    att::TStat::Decl { span: span.clone(), vartype: vartype.clone(), expr: None }
                };
                self.scopes.put(name.name.clone(), vartype, span.clone());

                output
            },
            ast::Statement::Body { content, span } => todo!(),
            ast::Statement::IfStatement { cond, body, child, span } => todo!(),
            ast::Statement::Assignment { span, target, expr } => todo!(),
            ast::Statement::Expr { span, expr } => todo!(),
        }
    }

    fn check_type(&mut self, node: &ast::VarType) -> att::Type {
        match node {
            ast::VarType::Normal(span, name) => {
                if let Some(vartype) = self.scopes.get_type(name) {
                    vartype
                } else {
                    self.err.error(ErrorKind::UndefinedType, span.clone());
                    att::Type::Infer
                }
            },
            ast::VarType::Array(span, items, length) => todo!(),
            ast::VarType::Pointer(span, item) => todo!(),
            ast::VarType::Func(_, _, _) => todo!(),
        }
    }

    fn check_expr(&mut self, node: &ast::Expression) -> att::TExpr {
        match node {
            &ast::Expression::Number(span, num) => att::TExpr::Uint { span, value: num },
            ast::Expression::Identifier(id) => {
                if let Some(variable) = self.scopes.get(&id.name) {
                    att::TExpr::Var { span: id.span, vartype: variable.vartype.clone() }
                } else {
                    self.err.error(ErrorKind::UndefinedVariable, id.span);
                    att::TExpr::Var { span: id.span, vartype: att::Type::Infer }
                }
            },
            ast::Expression::BinaryOp(span, left, op, right) => {
                let left = self.check_expr(left);
                let right = self.check_expr(right);

                let vartype = if left.vartype() == right.vartype() {match op {
                    ast::Op::CondEq(_)
                        | ast::Op::CondG(_) | ast::Op::CondGEq(_)
                        | ast::Op::CondL(_) |ast::Op::CondLEq(_)
                        => att::Type::Bool,
                     _ => left.vartype().clone()
                }} else {
                    self.err.error(ErrorKind::UnexpectedType { expected: left.vartype().clone() }, span.clone());
                    att::Type::Infer
                };

                att::TExpr::BinaryOp { span: span.clone(), left: Box::new(left), op: op.clone(), right: Box::new(right), vartype }
            },
        }
    }
}
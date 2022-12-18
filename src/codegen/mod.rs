pub mod asm;
pub mod allocator;
pub mod rustcompile;

use crate::ast;

pub struct CodeGen {
    allocator: allocator::Allocator,
    pub asm: asm::URCLAsm,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            allocator: allocator::Allocator::new(10),
            asm: asm::URCLAsm::new(),
        }
    }

    pub fn gen(&mut self, ast_nodes: &[ast::Statement]) {
        println!("\nAllocator Debug");
        println!("==================================");
        self.allocator.init(ast_nodes);
        for node in ast_nodes {
            self.gen_stmt(node);
        }
    }

    pub fn gen_stmt(&mut self, node: &ast::Statement) {
        match node {
            ast::Statement::Declaration {span, vartype, target, expr } => {
                let var_reg = self.allocator.get_var(span.start().lineno, &target.name);
                self.gen_expr(expr, Some(var_reg)); //value should be in var reg
                self.allocator.done_with_var(&mut self.asm, span.start().lineno, &target.name, var_reg);
            },
            ast::Statement::Assignment { span, target, expr } => {
                match target {
                    ast::Expression::Identifier(identifier) => {
                        let var_reg = self.allocator.get_var(span.start().lineno, &identifier.name);
                        self.gen_expr(expr, Some(var_reg)); //value should be in var reg
                        self.allocator.done_with_var(&mut self.asm, span.start().lineno, &identifier.name, var_reg);
                    },
                    _ => todo!(),
                }
            },
            ast::Statement::Body { content, span } => todo!(),
            ast::Statement::IfStatement { cond, body, child, span } => todo!(),
            ast::Statement::Expr { span, expr } => todo!(),
        }
    }

    pub fn gen_expr(&mut self, node: &ast::Expression, reg: Option<usize>) -> usize {
        match node {
            ast::Expression::Number(span, value) => {
                if let Some(dest_reg) = reg {
                    self.asm.put_li(dest_reg, *value);
                    return dest_reg;
                }
                else {
                    let new_reg = self.allocator.get_empty_reg(span.start().lineno);
                    self.asm.put_li(new_reg, *value);
                    return new_reg;
                }
            },
            ast::Expression::Identifier(identifier) => {
                let reg_loaded: usize = self.allocator.get_var_loaded(&mut self.asm, identifier.span.start().lineno, &identifier.name);
                if let Some(dest_reg) = reg {
                    if reg_loaded != dest_reg {
                        self.asm.put_mov(dest_reg, reg_loaded);
                        return dest_reg;
                    }
                }
                return reg_loaded;
            },
            ast::Expression::BinaryOp(_span, expr1, op, expr2) => {
                let reg1: usize = self.gen_expr(&expr1, reg);
                let reg2: usize = self.gen_expr(&expr2, Option::None);

                let mut dest: usize = reg1;
                if let Some(dest_reg) = reg{
                    dest = dest_reg;
                }

                match op {
                    ast::Op::Add(_) => self.asm.put_add(dest, reg1, reg2),
                    ast::Op::Sub(_) => self.asm.put_sub(dest, reg1, reg2),
                    ast::Op::Mult(_) => self.asm.put_mlt(dest, reg1, reg2),
                    ast::Op::Div(_) => self.asm.put_div(dest, reg1, reg2),
                    _ => todo!(),
                }
                return dest;
            },
        }
    }
}
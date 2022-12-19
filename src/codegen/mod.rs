pub mod asm;
pub mod allocator;
pub mod rustcompile;

use crate::{ast, span};

pub struct CodeGen {
    allocator: allocator::Allocator,
    pub asm: asm::URCLAsm,
    label: u32,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            allocator: allocator::Allocator::new(10),
            asm: asm::URCLAsm::new(),
            label: 0,
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
            ast::Statement::Body { content, span } => self.gen_body(content),
            ast::Statement::IfStatement { span: _, cond, body, child } => self.gen_if(Option::None, cond, body, child),
            ast::Statement::Expr { span, expr } => {self.gen_expr(expr, Some(0));},
        }
    }
    //holy horrendous, i just direct used the old js version for this
    fn gen_if(&mut self, end_label: Option<String>, c_cond: &ast::Expression, c_body: &Box<ast::Statement>, c_child: &Box<Option<ast::Statement>>) {
        let mut label = self.gen_label();
        let mut end_label = end_label;
        let unbox_child = &*(*c_child);
        if let Some(c) = &*unbox_child {
            if end_label == Option::None {
                end_label = Some(self.gen_label());
            }
        }
        else {
            if end_label != Option::None {
                label = end_label.clone().unwrap();
            }
        }
        self.gen_cond(c_cond, &label);
        match &**c_body {
            ast::Statement::Body { span: _, content } => {
                self.gen_body(content);
            },
            _ => {

            }
        }
        if let Some(s) = &*unbox_child {
            if let Some(lbl) = &end_label {
                self.asm.put_jmp(lbl);
            }
        }
        self.asm.put_label(&label);

        if let Some(child_stmt) = &*unbox_child {
            match child_stmt {
                ast::Statement::Body { span: _, content } => {
                    self.gen_body(&content);
                    if let Some(lbl) = &end_label {
                        self.asm.put_label(lbl);
                    }
                }
                ast::Statement::IfStatement { span: _, cond, body, child } => {
                    self.gen_if(end_label, cond, body, child);
                }
                _ => {}
            }
        }
    }

    fn gen_body(&mut self, content: &Vec<Box<ast::Statement>>) {
        for stmt in content {
            self.gen_stmt(&*stmt);
        }
    }

    fn gen_label(&mut self) -> String {
        self.label += 1;
        return format!(".LABEL_{}", self.label);
    }

    fn gen_cond(&mut self, cond: &ast::Expression, end_label: &String) {
        match cond {
            ast::Expression::BinaryOp(span, expr1, op, expr2) => {
                let reg1: usize = self.gen_expr(&(*expr1), Option::None);
                let reg2: usize = self.gen_expr(&(*expr2), Option::None);
                match op {
                    ast::Op::CondEq(_) => self.asm.put_branch("BNE", end_label, reg1, reg2),
                    ast::Op::CondG(_) => self.asm.put_branch("BLE", end_label, reg1, reg2),
                    ast::Op::CondGEq(_) => self.asm.put_branch("BRL", end_label, reg1, reg2),
                    ast::Op::CondL(_) => self.asm.put_branch("BGE", end_label, reg1, reg2),
                    ast::Op::CondLEq(_) => self.asm.put_branch("BRG", end_label, reg1, reg2),
                    _ => {
                        //Type checker should do this
                        panic!("Invalid conditional operator");
                    }
                }
            }
            _ => {
                return;
            }
        }
    }

    pub fn gen_expr(&mut self, node: &ast::Expression, reg: Option<usize>) -> usize {
        match node {
            ast::Expression::Number(span, value) => {
                if let Some(dest_reg) = reg {
                    self.asm.put_li(dest_reg, *value);
                    return dest_reg;
                }
                let new_reg = self.allocator.get_empty_reg(span.start().lineno);
                self.asm.put_li(new_reg, *value);
                return new_reg;
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
                let reg1: usize = self.gen_expr(&expr1, Option::None);
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
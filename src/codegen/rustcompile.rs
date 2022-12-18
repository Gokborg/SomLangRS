use std::{path::Path, fs::File, process::Command, io::Write};
use crate::ast;

pub struct RustGenerator {
    output: String
}

impl RustGenerator {
    pub fn gen(ast_nodes: &[ast::Statement]) {
        let mut gen = Self {output: String::new()};
        gen.output.push_str("#[allow(unused_variables, unused_unsafe)]\nfn main(){unsafe{\n");
        for node in ast_nodes {
            let stmt = gen.gen_stmt(node);
            gen.output.push_str(&stmt);
            gen.output.push('\n');
        }
        gen.output.push('}');
        gen.output.push('}');
        compile_rust(Path::new("somoutput"), &gen.output);
    }

    pub fn gen_stmt(&mut self, node: &ast::Statement) -> String {
        match node {
            ast::Statement::Declaration {span: _, vartype, name, expr } => {
                match vartype {
                    ast::VarType::Normal(_span, vartype_str) => {
                        if vartype_str == "uint" {
                            return format!("let mut {}: {} = {};", name, "u8", self.gen_expr(expr));
                        }
                        panic!("Only support uint for rust compilation! do not use other types!");
                    }
                    _ => {
                        todo!();
                    }
                }
            },
            ast::Statement::Assignment {span: _, name, expr } => {
                return format!("{} = {};", name, self.gen_expr(expr));
            },
            ast::Statement::Body { content, span: _ } => {
                let mut result: String = String::new();
                for stmt in content {
                    result.push_str(&self.gen_stmt(&(*stmt)));
                }
                return format!("{{\n{}\n}}", result);
            },
            ast::Statement::IfStatement {span: _, cond, body, child: _ } => {
                return format!("if {} {}", self.gen_expr(cond), self.gen_stmt(&(*body)));
            },
        }
    }

    pub fn gen_expr(&mut self, node: &ast::Expression) -> String {
        match node {
            ast::Expression::Number(_span, value) => {
                return format!("{}", value);
            },
            ast::Expression::Identifier(_span, value) => {
                return value.clone();
            },
            ast::Expression::BinaryOp(_span, expr1, op, expr2) => {
                let op_str: &str;
                match op {
                    ast::Op::Add(_) => op_str = "+",
                    ast::Op::Sub(_) => op_str = "-",
                    ast::Op::Mult(_) => op_str = "*",
                    ast::Op::Div(_) => op_str = "/",
                    ast::Op::CondEq(_) => op_str = "==",
                    ast::Op::CondG(_) => op_str = ">",
                    ast::Op::CondGEq(_) => op_str = ">=",
                    ast::Op::CondL(_) => op_str = "<",
                    ast::Op::CondLEq(_) => op_str = "<=",
                }
                return self.gen_expr(expr1) + op_str + &self.gen_expr(expr2)
            },
        }
    }

    
}


fn compile_rust(outdir: &Path, program: &str) {
    let rust_filename =  &outdir.join("program.rs");
    let exe_filename = &outdir.join("program");

    let mut rust_file = File::create(rust_filename).unwrap();
    write!(rust_file, "{}", program).unwrap();
    drop(rust_file);

    let output = Command::new("rustc")
        .arg(rust_filename)
        .arg("--out-dir").arg(outdir)
        .output()
        .expect("failed to compile program");

    println!("{:?}", output);

    let output = Command::new(exe_filename)
        .output()
        .expect("failed to run program");
    
    println!("{:?}", output);
}
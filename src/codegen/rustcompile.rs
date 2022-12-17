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
            ast::Statement::Declaration {span, vartype, name, expr } => {
                match vartype {
                    ast::VarType::Normal(span, vartype_str) => {
                        return format!("let {}: {} = {};", name, vartype_str, self.gen_expr(expr));
                    }
                    _ => {
                        todo!();
                    }
                }
            },
            ast::Statement::Assignment { span, name, expr } => {
                todo!()
            },
        }
    }

    pub fn gen_expr(&mut self, node: &ast::Expression) -> String {
        match node {
            ast::Expression::Number(span, value) => {
                return format!("{}", value);
            },
            ast::Expression::Identifier(span, value) => {
                return value.clone();
            },
            ast::Expression::BinaryOp(_span, expr1, op, expr2) => {
                let op_str: &str;
                match op {
                    ast::Op::Add(_) => op_str = "+",
                    ast::Op::Sub(_) => op_str = "-",
                    ast::Op::Mult(_) => op_str = "*",
                    ast::Op::Div(_) => op_str = "/",
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
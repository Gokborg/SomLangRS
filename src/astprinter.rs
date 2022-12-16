use super::ast;

pub fn print_ast(nodes: &Vec<ast::Statement>) {
    for node in nodes {
        p(node, 0);
    }
}

fn p(node: &ast::Statement, lvl: u32) {
    let mut indent = String::new();
    for _i in 0..lvl {
        indent.push('\t');
    }
    match node {
        ast::Statement::Declaration {
            start: _, 
            vartype, 
            name, 
            expr } => {
                println!("{}Declaration", indent);
                p_vartype(vartype, lvl+1);
                let t = indent + " ";
                println!("{}├Name({})", t , name);
                println!("{}└Expression:", t);
                p_expr(expr, lvl+2);
        }
        _ => {

        }
    }
}

fn p_vartype(node: &ast::VarType, lvl: u32) {
    let mut indent = String::new();
    for _i in 0..lvl {
        indent.push(' ');
    }
    match node {
        ast::VarType::Normal(tok) => {
            println!("{}├VarType({})", indent, tok.value);
        }
        _ => {

        }
    }
}

fn p_expr(node: &ast::Expression, lvl: u32) {
    let mut indent = String::new();
    for _i in 0..lvl {
        indent.push(' ');
    }
    match node {
        ast::Expression::Number(num, _) => {
            println!("{}├Number({})", indent, num);
        }
        ast::Expression::Identifier(ident, _) => {
            println!("{}├Identifier({})", indent, ident);
        }
        _ => {

        }
    }
}

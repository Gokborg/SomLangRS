use super::ast;

pub fn print_ast(nodes: &Vec<ast::Statement>) {
    for node in nodes {
        p(node, 0);
    }
}

fn p(node: &ast::Statement, lvl: u32) {
    let mut indent = String::new();
    for _i in 0..lvl {
        indent.push(' ');
    }
    match node {
        ast::Statement::Declaration {
            span: _, 
            vartype, 
            name, 
            expr } => {
                println!("{}├Declaration", indent);
                p_vartype(vartype, lvl+1);
                let t = indent + " ";
                println!("{}├Name({})", t , name);
                println!("{}└Expression:", t);
                p_expr(expr, lvl+2);
        }
        ast::Statement::Assignment { span, name, expr } => {
            println!("{}├Assignment", indent);
            let t = indent + " ";
            println!("{}├Name({})", t , name);
            println!("{}└Expression:", t);
            p_expr(expr, lvl+2);
        },
        ast::Statement::Body { span: _, content } => {
            println!("{}Body", indent);
            for stmt in content {
                p(&(*stmt), lvl+1);
            }
        },
        ast::Statement::IfStatement {span: _, cond, body, child: _} => {
            println!("{}├If", indent);
            p_expr(cond, lvl);
            p(body, lvl+1);
        },
    }
}

fn p_vartype(node: &ast::VarType, lvl: u32) {
    let mut indent = String::new();
    for _i in 0..lvl {
        indent.push(' ');
    }
    match node {
        ast::VarType::Normal(_, value) => {
            println!("{}├VarType({})", indent, value);
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
        ast::Expression::Number(_, value) => {
            println!("{}{}-Number({})", indent, lvl, value);
        }
        ast::Expression::Identifier(_, ident) => {
            println!("{}{}-Identifier({})", indent, lvl, ident);
        }
        ast::Expression::BinaryOp(_, expr1, op, expr2) => {
            println!("{}{}-Op({:?})", indent, lvl, op);
            p_expr(expr1, lvl+1);
            p_expr(expr2, lvl+1);
        }
    }
}

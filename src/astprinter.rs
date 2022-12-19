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
            target, 
            expr } => {
                println!("{}├Declaration", indent);
                p_vartype(vartype, lvl+1);
                let t = indent + " ";
                println!("{}├Name({})", t , target.name);
                println!("{}└Expression:", t);
                p_expr(expr, lvl+2);
        }
        ast::Statement::Assignment { span, target, expr } => {
            println!("{}├Assignment", indent);
            let t = indent + " ";
            println!("{}├Name(", t);
            p_expr(target, lvl+2);
            println!("{})", t);
            println!("{}└Expression:", t);
            p_expr(expr, lvl+2);
        },
        ast::Statement::Body { span: _, content } => {
            println!("{}├Body", indent);
            for stmt in content {
                p(&(*stmt), lvl+1);
            }
        },
        ast::Statement::IfStatement {span: _, cond, body, child} => {
            println!("{}├If", indent);
            println!("{} ├Cond", indent);
            p_expr(cond, lvl+2);
            p(body, lvl+1);
            println!("{} ├Child", indent);
            if let Some(stmt) = &*(*child) {
                p(stmt, lvl+2);
            }
        },
        ast::Statement::Expr { span, expr } => {
            println!("{}├Expr", indent);
            p_expr(expr, lvl+1);
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
        ast::Expression::Identifier(id) => {
            println!("{}{}-Identifier({})", indent, lvl, id.name);
        }
        ast::Expression::BinaryOp(_, expr1, op, expr2) => {
            println!("{}{}-Op({:?})", indent, lvl, op);
            p_expr(expr1, lvl+1);
            p_expr(expr2, lvl+1);
        }
    }
}

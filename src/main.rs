mod token;
mod lexer;
mod ast;
mod parsers;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new();
    let mut tokens = lexer.lex(vec![
        "let a: uint = 3;".to_string(),
        "let a: uint = b;".to_string(),
    ]);
    //Filters out whitespaces
    tokens = tokens.into_iter().filter(|x| x.kind != token::Kind::WHITESPACE).collect();
    println!("LEXER");
    println!("==================================");
    //Print tokens of lexer
    for tok in &tokens {
        println!("{:?}", tok);
    }
    println!("\nPARSER");
    println!("==================================");
    //Parser
    let mut parser = parser::Parser::new(tokens.as_slice());
    let ast_nodes = parser.parse();
    // for ast_node in &ast_nodes {
    //     println!("{:?}", ast_node);
    // }

    let lvl: u32 = 0;
    for node in &ast_nodes {
        p(node, lvl)
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

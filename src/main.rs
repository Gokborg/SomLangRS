

mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new();
    let mut tokens = lexer.lex(vec![
        "let a: uint = 5;".to_string(),
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
    for ast_node in &ast_nodes {
        println!("{:?}", ast_node);
    }
}

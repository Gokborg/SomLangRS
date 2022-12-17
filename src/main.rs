mod token;
mod lexer;
mod ast;
mod parsers;
mod parser;
mod astprinter;
mod codegen;
mod span;
mod errorcontext;

fn main() {
    let mut lexer = lexer::Lexer::new();
    
    let mut tokens = lexer.lex(vec![
        "let a: u8 = 5;".to_string(),
        "let b: u8 = a;".to_string(),
    ]);
    //Filters out whitespaces
    tokens = tokens.into_iter().filter(|x| x.kind != token::Kind::WHITESPACE).collect();
    println!("LEXER");
    println!("==================================");
    //Print tokens of lexer
    for tok in &tokens {
        println!("{}", tok);
    }
    println!("\nPARSER");
    println!("==================================");
    //Parser
    let mut parser = parser::Parser::new(tokens.as_slice());
    let ast_nodes = parser.parse();

    // for ast_node in &ast_nodes {
    //     println!("{:?}", ast_node);
    // }

    astprinter::print_ast(&ast_nodes);

    let mut codegen = codegen::CodeGen::new();
    codegen.gen(&ast_nodes);
    println!("\nASM");
    println!("==================================");
    println!("{}", codegen.asm);

}


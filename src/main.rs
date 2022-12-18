#[allow(unused_variables)]
mod token;
mod lexer;
mod ast;
mod parsers;
mod parser;
mod astprinter;
mod codegen;
mod span;
mod errorcontext;
mod typechecking;

use crate::{typechecking::TypeChecker, errorcontext::ErrorContext};

fn main() {
    // create target folder
    std::fs::create_dir("somoutput").unwrap_or(println!("Unable to create rust compile directory"));

    //reads from test.som in examples folder
    let contents: Vec<String> = std::fs::read_to_string("examples/test.som")
        .expect("Couldn't read test.som file in examples folder")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();

    let mut lexer = lexer::Lexer::new();
    
    let mut tokens = lexer.lex(contents);
    let mut error_context = ErrorContext::new();

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

    // let checker = TypeChecker::check(&mut error_context, &ast_nodes);
    // println!("{:?}", checker);

    astprinter::print_ast(&ast_nodes);

    println!("\nRUST COMPILED OUTPUT");
    println!("==================================");
    codegen::rustcompile::RustGenerator::gen(&ast_nodes);

    let mut codegen = codegen::CodeGen::new();
    codegen.gen(&ast_nodes);
    println!("\nASM");
    println!("==================================");
    println!("{}", codegen.asm);
    
}


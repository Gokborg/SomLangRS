pub mod token;
pub mod lexer;
pub mod ast;
pub mod parsers;
pub mod parser;
pub mod astprinter;
pub mod codegen;
pub mod span;
pub mod errorcontext;
pub mod typechecking;
pub mod ansi;
use wasm_bindgen::prelude::*;

use crate::{errorcontext::ErrorContext, typechecking::TypeChecker};

#[wasm_bindgen]
pub fn wasm_to_wat(bytes: &[u8]) -> String {
    wasmprinter::print_bytes(bytes).unwrap()
}

#[wasm_bindgen]
pub fn compile(src: &str) -> Vec<u8> {
    // create target folder
    //reads from test.som in examples folder
    let lines: Vec<&str> = src.lines().collect();
    
    let mut tokens = lexer::Lexer::new().lex(src);
    let mut error_context = ErrorContext::new(&lines);

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
    let mut parser = parser::Parser::new(tokens.as_slice(), &mut error_context);
    let ast_nodes = parser.parse();

    // for ast_node in &ast_nodes {
    //     println!("{:?}", ast_node);
    // }
    astprinter::print_ast(&ast_nodes);
    
    
    println!("\nTYPECHECKER");
    println!("==================================");
    let (checker, att) = TypeChecker::check(parser.err, &ast_nodes);
    for stat in att.iter() {
        println!("{:?}", stat);
    }
    println!("\nERRORS:");
    println!("{}", checker.err);

    codegen::wasm::WasmGen::gen(&att)
    
    // println!("\nRUST COMPILED OUTPUT");
    // println!("==================================");
    // codegen::rustcompile::RustGenerator::gen(&ast_nodes);

    // let mut codegen = codegen::CodeGen::new();
    // codegen.gen(&ast_nodes);
    // println!("\nASM");
    // println!("==================================");
    // println!("{}", codegen.asm);
    
}
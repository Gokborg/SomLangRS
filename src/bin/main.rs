use somelang_rs::*;
use somelang_rs::{typechecking::TypeChecker, errorcontext::ErrorContext};

fn main() {
    // create target folder
    std::fs::create_dir("somoutput").unwrap_or(println!("Unable to create rust compile directory"));

    //reads from test.som in examples folder
    let contents: Vec<String> = std::fs::read_to_string("examples/wasm.som")
        .expect("Couldn't read test.som file in examples folder")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut lexer = lexer::Lexer::new();
    
    let mut tokens = lexer.lex(&contents);
    let mut error_context = ErrorContext::new(&contents);

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

    // codegen::wasm::WasmGen::gen(&att);
    
    println!("\nRUST COMPILED OUTPUT");
    println!("==================================");
    codegen::rustcompile::RustGenerator::gen(&ast_nodes);

    let mut codegen = codegen::CodeGen::new();
    codegen.gen(&ast_nodes);
    println!("\nASM");
    println!("==================================");
    println!("{}", codegen.asm);
    
}
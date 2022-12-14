mod token;
mod lexer;

fn main() {
    let mut lexer: lexer::Lexer = lexer::Lexer::new();
    let tokens = lexer.lex(vec![
        "let a: uint = 5".to_string(),
        "uint b == 23;".to_string(),
    ]);
    for tok in tokens {
        println!("{:?}", tok);
    }
}

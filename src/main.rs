mod token;
mod lexer;

fn main() {
    let mut lexer: lexer::Lexer = lexer::Lexer::new();
    let tokens = lexer.lex(vec![
        "uint a = 55;".to_string(),
        "uint b == 23;".to_string(),
    ]);
    for tok in tokens {
        println!("{:?}", tok);
    }
}

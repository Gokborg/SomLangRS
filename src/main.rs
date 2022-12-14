mod token;
mod lexer;

fn main() {
    let mut lexer: lexer::Lexer = lexer::Lexer::new();
    let tokens = lexer.lex(vec![
        "bad boys in paris123".to_string()
    ]);
    for tok in tokens {
        println!("{:?}", tok);
    }
}
